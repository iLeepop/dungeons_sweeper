use bevy::ecs::observer::On;
use bevy::log;
use bevy::prelude::*;

use crate::AppState;
use crate::bundles::enemy_neighbor_bundle;
use crate::components::Exposed;
use crate::components::TriggerRemaining;
use crate::components::{
    Coordinates, Damage, Enemy, EnemyNeighbor, GoldCoin, Grass, Health, OutWay, Player, Safe,
    Treasure, Uncover,
};
use crate::effects::EffectCounters;
use crate::effects::EffectPhase;
use crate::effects::EffectPhaseMessage;
use crate::events::enemy_event::EnemyAttackEvent;
use crate::events::taggle::ToggleEvent;
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::tile::Tile;
use crate::resources::tile_map::enemy_neighbor_display_label;
use crate::resources::{PendingBoardRebuild, PlayerOptions};

pub fn taggle_consumer(
    event: On<ToggleEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_option: Res<BoardOption>,
    player_options: Res<PlayerOptions>,
    mut pending_board: ResMut<PendingBoardRebuild>,
    mut next_state: ResMut<NextState<AppState>>,
    mut effect_counters: ResMut<EffectCounters>,
    mut effect_phase_writer: MessageWriter<EffectPhaseMessage>,
    tile_type: Query<(
        Option<&Enemy>,
        Option<&EnemyNeighbor>,
        Option<&Grass>,
        Option<&Treasure>,
        Option<&OutWay>,
        Option<&Safe>,
    )>,
    mut trigger_remaining: Query<&mut TriggerRemaining>,
    // Bevy 无法证明「敌方 Health」与「玩家 Health」互斥，必须放进同一 ParamSet（错误 B0001）。
    mut health_queries: ParamSet<(
        Query<&mut Health, With<Enemy>>,
        Query<&Health, With<Enemy>>,
        Query<(&mut Health, &mut Damage, &mut GoldCoin), With<Player>>,
    )>,
    player_entity: Single<Entity, With<Player>>,
    parent: Query<&ChildOf>,
    children_q: Query<&Children>,
    mut text2d_q: Query<&mut Text2d>,
) {
    // if let Some(tile) = board.tiles.get(&event.0) {
    //     commands.entity(*tile).despawn();
    // }
    if let Some(cover) = board.covers.get(&event.0) {
        #[cfg(feature = "debug")]
        log::info!("despawn cover: {:?}", *cover);
        let parent = match parent.get(*cover) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error getting parent: {:?}", e);
                return;
            }
        };
        commands.entity(parent.0).insert(Exposed);
        commands.entity(*cover).insert(Uncover);
        board.covers.remove(&event.0);
        commands.trigger(EnemyAttackEvent);
        return;
    }
    let Some(tile_ent) = board.tiles.get(&event.0).copied() else {
        return;
    };

    #[cfg(feature = "debug")]
    log::info!("despawn tile: {:?}", tile_ent);

    let (enemy, enemy_neighbor, grass, treasure, out_way, safe) = match tile_type.get(tile_ent) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error getting tile: {:?}", e);
            return;
        }
    };

    // 检查触发剩余次数
    if let Ok(mut remain) = trigger_remaining.get_mut(tile_ent) {
        if !remain.try_consume_one() {
            log::info!("tile is triggered over times");
            return;
        } else {
            log::info!("tile triggered");
        }
    }

    // ---------------------------------------------------------------------------
    // 出口：升关、刷新 BoardOption 计数，帧末重建棋盘并暂停（见 [`crate::flush_pending_board_rebuild`]）
    // ---------------------------------------------------------------------------
    if out_way.is_some() {
        // 升关与 `BoardOption` 刷新在帧末 [`crate::flush_pending_board_rebuild`]，减轻 Observer 参数数量。
        pending_board.0 = true;
        next_state.set(AppState::GamePause);
        return;
    }

    // ---------------------------------------------------------------------------
    // 安全点：按 [`PlayerOptions::safe_heal_per_trigger`] 回血，不超过生命上限
    // ---------------------------------------------------------------------------
    if safe.is_some() {
        if let Ok((mut hp, _, _)) = health_queries.p2().get_mut(*player_entity) {
            let add = player_options.safe_heal_per_trigger as i32;
            let cap = player_options.max_hp as i32;
            hp.0 = (hp.0 as i32 + add)
                .min(cap)
                .clamp(i8::MIN as i32, i8::MAX as i32) as i8;
        }
    }

    // ---------------------------------------------------------------------------
    // 宝藏：永久增加攻击力（饱和加法）
    // ---------------------------------------------------------------------------
    if treasure.is_some() {
        if let Ok((_, mut dmg, _)) = health_queries.p2().get_mut(*player_entity) {
            dmg.0 = dmg
                .0
                .saturating_add(player_options.treasure_damage_bonus);
        }
    }

    // ---------------------------------------------------------------------------
    // 玩家攻击敌方格（须在效果广播之前结算，避免与其它 System 对 [`Health`] 的写入顺序混淆）
    // ---------------------------------------------------------------------------
    if enemy.is_some() {
        let coord = event.0;
        let player_atk = match health_queries.p2().get(*player_entity) {
            Ok((_, dmg, _)) => dmg.0,
            Err(_) => {
                log::error!("player has no Damage component");
                return;
            }
        };
        let killed_opt = match health_queries.p0().get_mut(tile_ent) {
                Ok(mut enemy_hp) => {
                    apply_player_damage_to_enemy_health(&mut enemy_hp, player_atk);
                    #[cfg(feature = "debug")]
                    log::info!(
                        "player attacks enemy: atk={}, enemy hp={}",
                        player_atk,
                        enemy_hp.0,
                    );
                    Some(enemy_hp.0 <= 0)
                }
                Err(_) => {
                    log::error!("enemy tile missing Health");
                    None
                }
            };

            if let Some(killed) = killed_opt {
                if killed {
                let hp_sum_neighborhood =
                    board.adjacent_enemy_hp_sum_from_entities(coord, &health_queries.p1());
                let stored = hp_sum_neighborhood.min(u16::MAX as u32) as u16;
                if let Some(cell) = board.tile_map.get_tile_mut(coord) {
                    *cell = Tile::EnemyNeighbor(stored);
                } else {
                    log::error!("tile_map missing coord {:?}", coord);
                }

                commands.entity(tile_ent).despawn();

                let board_size = Vec3::new(
                    (board.tile_map.width() * board.tile_size.width) as f32,
                    (board.tile_map.height() * board.tile_size.height) as f32,
                    0.0,
                );

                if let Some(board_parent) = board.board_entity {
                    let new_ent = commands
                        .spawn(enemy_neighbor_bundle(
                            coord,
                            board.tile_size,
                            board_option.padding,
                            board_size,
                            hp_sum_neighborhood,
                            &board_option.counter_font,
                        ))
                        .id();
                    commands.entity(new_ent).insert(ChildOf(board_parent));

                    board.tiles.insert(coord, new_ent);

                    if let Ok((_, _, mut gold)) = health_queries.p2().get_mut(*player_entity) {
                        gold.0 = gold.0.saturating_add(1);
                    }
                } else {
                    log::error!("board_entity missing, cannot spawn replacement tile");
                    board.tiles.remove(&coord);
                }
                }

                // --- 刷新周围 EnemyNeighbor：展示值为邻格敌方实体 HP 之和 ---
                refresh_enemy_neighbor_displays_around(
                    coord,
                    &mut board,
                    &health_queries.p1(),
                    &children_q,
                    &mut text2d_q,
                );
            }
    }

    // --- 效果系统：格子触发计数 + 广播「玩家已触发该格」阶段（具体执行在 PostUpdate） ---
    effect_counters.player_tile_triggers = effect_counters
        .player_tile_triggers
        .saturating_add(1);
    effect_phase_writer.write(EffectPhaseMessage {
        phase: EffectPhase::AfterPlayerTileTrigger,
        coord: Some(event.0),
        tile: Some(tile_ent),
    });

    if enemy_neighbor.is_some() {
        log::info!("you get on enemy neighbor");
    }

    if grass.is_some() {
        log::info!("you get on grass wuth health increase");
    }

    commands.trigger(EnemyAttackEvent);
}

// ---------------------------------------------------------------------------
// 战斗：直接扣除敌方 [`Health`]（敌方无护盾组件）
// ---------------------------------------------------------------------------

fn apply_player_damage_to_enemy_health(health: &mut Health, atk: u8) {
    let cur = i32::from(health.0);
    let dmg = i32::from(atk);
    health.0 = (cur - dmg).clamp(i8::MIN as i32, i8::MAX as i32) as i8;
}

// ---------------------------------------------------------------------------
// 刷新攻击点周围及自身的 EnemyNeighbor（展示 = 邻格敌方实体 HP 之和）
// ---------------------------------------------------------------------------

fn refresh_enemy_neighbor_displays_around(
    center: Coordinates,
    board: &mut Board,
    enemy_hp_read: &Query<&Health, With<Enemy>>,
    children_q: &Query<&Children>,
    text2d_q: &mut Query<&mut Text2d>,
) {
    let coords: Vec<Coordinates> = board
        .tile_map
        .safe_square_at(center)
        .chain(std::iter::once(center))
        .collect();

    for coord in coords {
        let Some(entity) = board.tiles.get(&coord).copied() else {
            continue;
        };
        let Some(tile_kind) = board.tile_map.get_tile(coord) else {
            continue;
        };
        if !matches!(tile_kind, Tile::EnemyNeighbor(_)) {
            continue;
        }
        let hp_sum = board.adjacent_enemy_hp_sum_from_entities(coord, enemy_hp_read);
        let stored = hp_sum.min(u16::MAX as u32) as u16;
        if let Some(cell) = board.tile_map.get_tile_mut(coord) {
            *cell = Tile::EnemyNeighbor(stored);
        }
        let Ok(children) = children_q.get(entity) else {
            continue;
        };
        for child in children.iter() {
            if let Ok(mut text) = text2d_q.get_mut(child) {
                *text = Text2d::new(enemy_neighbor_display_label(hp_sum));
            }
        }
    }
}
