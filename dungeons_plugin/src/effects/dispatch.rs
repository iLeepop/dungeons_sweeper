//! 效果调度：通过 Bevy [`Message`] 将 Observer 与「真正跑 Query 的 System」解耦。
//!
//! ## 主流做法对比（摘要）
//! - **单通道 Message + 集中 System（当前）**：易读、顺序清晰；每多一种时间线要加一个 phase 并在某处 write。
//! - **每种时机独立 Message**：解耦极强，类型与 `add_message` 变多。
//! - **Custom(Box<dyn Fn…>) 触发条件**：扩展极强，调试与对象安全成本高。
//!
//! ## 与格子「还能不能触发」的关系
//! [`crate::components::TriggerRemaining`] 仍由游戏逻辑独占；本模块只处理「逻辑已决定进入某 phase」之后的效果链。

use bevy::prelude::*;

use crate::components::Damage;
use crate::components::Enemy;
use crate::components::Player;
use crate::components::coordinates::Coordinates;
use crate::components::entity_status::Health;
use crate::effects::context::{
    EnemyTileEffectContext, PlayerEffectContext, TileEffectContext, WorldEffectContext,
};
use crate::effects::counters::EffectCounters;
use crate::effects::loaders::{
    EnemyEffectLoader, PlayerEffectLoader, TileEffectLoader, WorldEffectHost, WorldEffectLoader,
};
use crate::effects::trigger::EffectPhase;
use crate::resources::PlayerOptions;
use crate::resources::board::Board;

// ---------------------------------------------------------------------------
// 跨系统传递的调度消息
// ---------------------------------------------------------------------------

/// 由 Observer 或其它 System 写入；[`effect_phase_dispatch_system`] 在同一帧稍后消费。
#[derive(Clone, Message, Debug)]
pub struct EffectPhaseMessage {
    /// 当前所处的效果阶段。
    pub phase: EffectPhase,
    /// 与格子相关的坐标（例如玩家点击格）。
    pub coord: Option<Coordinates>,
    /// 与格子相关的实体（地图上的 tile 根实体）。
    pub tile: Option<Entity>,
}

// ---------------------------------------------------------------------------
// 调度队列：合并四类加载器后按 priority 与稳定次序排序
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum EffectWorkItem {
    World { host: Entity, index: usize },
    Player { index: usize },
    Tile { tile: Entity, index: usize },
    Enemy { tile: Entity, index: usize },
}

#[derive(Debug)]
struct ScoredWork {
    priority: i16,
    tie_break: usize,
    work: EffectWorkItem,
}

/// 从某一加载器的 `entries` 中挑出匹配 trigger 的索引，并压入全局队列。
fn collect_from_loader(
    entries: &[crate::effects::entry::EffectEntry],
    phase: EffectPhase,
    counters: &EffectCounters,
    tie_start: &mut usize,
    mut push: impl FnMut(ScoredWork),
    map_index: impl Fn(usize) -> EffectWorkItem,
) {
    for (index, entry) in entries.iter().enumerate() {
        if entry.trigger.matches(phase, counters) {
            let tie = *tie_start;
            *tie_start += 1;
            push(ScoredWork {
                priority: entry.priority,
                tie_break: tie,
                work: map_index(index),
            });
        }
    }
}

/// 消费本帧所有 [`EffectPhaseMessage`]，对每条消息合并四类加载器并执行。
pub fn effect_phase_dispatch_system(
    mut reader: MessageReader<EffectPhaseMessage>,
    board: Res<Board>,
    player_options: Res<PlayerOptions>,
    counters: Res<EffectCounters>,
    mut commands: Commands,
    player_entity: Single<Entity, With<Player>>,
    mut player_health_q: Query<&mut Health, With<Player>>,
    mut player_damage_q: Query<&mut Damage, With<Player>>,
    // 不包含敌方格：避免与玩家攻击敌方 Health 的写入冲突或误当作「地块回血」目标。
    mut tile_health_q: Query<&mut Health, (Without<Player>, Without<Enemy>)>,
    world_hosts: Query<(Entity, &WorldEffectLoader), With<WorldEffectHost>>,
    player_loader_q: Query<&PlayerEffectLoader, With<Player>>,
    tile_loader_q: Query<&TileEffectLoader>,
    enemy_loader_q: Query<&EnemyEffectLoader>,
) {
    // --- 逐条消费 phase 消息（通常每帧条数很少） ---
    for msg in reader.read() {
        let mut queue: Vec<ScoredWork> = Vec::new();
        let mut tie = 0usize;

        // --- 世界加载器：所有 host 上的条目 ---
        for (host_entity, loader) in world_hosts.iter() {
            collect_from_loader(
                &loader.entries,
                msg.phase,
                &counters,
                &mut tie,
                |scored| queue.push(scored),
                |index| EffectWorkItem::World {
                    host: host_entity,
                    index,
                },
            );
        }

        // --- 玩家加载器 ---
        let player_ent = *player_entity;
        if let Ok(loader) = player_loader_q.get(player_ent) {
            collect_from_loader(
                &loader.entries,
                msg.phase,
                &counters,
                &mut tie,
                |scored| queue.push(scored),
                |index| EffectWorkItem::Player { index },
            );
        }

        // --- 与具体格子绑定的 tile / enemy 加载器 ---
        if let Some(tile_e) = msg.tile {
            if let Ok(loader) = tile_loader_q.get(tile_e) {
                collect_from_loader(
                    &loader.entries,
                    msg.phase,
                    &counters,
                    &mut tie,
                    |scored| queue.push(scored),
                    |index| EffectWorkItem::Tile {
                        tile: tile_e,
                        index,
                    },
                );
            }
            if let Ok(loader) = enemy_loader_q.get(tile_e) {
                collect_from_loader(
                    &loader.entries,
                    msg.phase,
                    &counters,
                    &mut tie,
                    |scored| queue.push(scored),
                    |index| EffectWorkItem::Enemy {
                        tile: tile_e,
                        index,
                    },
                );
            }
        }

        // --- priority 降序；同 priority 按入队顺序（tie_break）升序 ---
        queue.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.tie_break.cmp(&b.tie_break))
        });

        // --- 顺序执行：每条效果单独拉取玩家生命等可变引用，避免跨条目别名借用 ---
        for item in queue {
            match item.work {
                EffectWorkItem::World { host, index } => {
                    let Ok((_, loader)) = world_hosts.get(host) else {
                        continue;
                    };
                    let Some(entry) = loader.entries.get(index) else {
                        continue;
                    };
                    let player_health = player_health_q.get_mut(player_ent).ok();
                    let mut ctx = WorldEffectContext {
                        commands: &mut commands,
                        host,
                        player: player_ent,
                        trigger_coord: msg.coord,
                        trigger_tile: msg.tile,
                        board: board.as_ref(),
                        player_health,
                    };
                    entry.behavior().apply_on_world(&mut ctx);
                }
                EffectWorkItem::Player { index } => {
                    let Ok(loader) = player_loader_q.get(player_ent) else {
                        continue;
                    };
                    let Some(entry) = loader.entries.get(index) else {
                        continue;
                    };
                    let player_health = player_health_q.get_mut(player_ent).ok();
                    let player_damage = player_damage_q.get_mut(player_ent).ok();
                    let mut ctx = PlayerEffectContext {
                        commands: &mut commands,
                        player: player_ent,
                        trigger_coord: msg.coord,
                        trigger_tile: msg.tile,
                        player_health,
                        player_damage,
                    };
                    entry.behavior().apply_on_player(&mut ctx);
                }
                EffectWorkItem::Tile { tile, index } => {
                    let Ok(loader) = tile_loader_q.get(tile) else {
                        continue;
                    };
                    let Some(entry) = loader.entries.get(index) else {
                        continue;
                    };
                    let Some(coord) = msg.coord else {
                        continue;
                    };
                    let player_health = player_health_q.get_mut(player_ent).ok();
                    let tile_health = tile_health_q.get_mut(tile).ok();
                    let mut ctx = TileEffectContext {
                        commands: &mut commands,
                        tile,
                        coord,
                        player: player_ent,
                        tile_health,
                        player_health,
                        player_hp_cap: player_options.max_hp,
                    };
                    entry.behavior().apply_on_tile(&mut ctx);
                }
                EffectWorkItem::Enemy { tile, index } => {
                    let Ok(loader) = enemy_loader_q.get(tile) else {
                        continue;
                    };
                    let Some(entry) = loader.entries.get(index) else {
                        continue;
                    };
                    let Some(coord) = msg.coord else {
                        continue;
                    };
                    let player_health = player_health_q.get_mut(player_ent).ok();
                    let tile_health = tile_health_q.get_mut(tile).ok();
                    let mut ctx = EnemyTileEffectContext {
                        commands: &mut commands,
                        tile,
                        coord,
                        player: player_ent,
                        tile_health,
                        player_health,
                    };
                    entry.behavior().apply_on_enemy_tile(&mut ctx);
                }
            }
        }
    }
}
