use bevy::ecs::observer::On;
use bevy::log;
use bevy::prelude::*;

use crate::bundles::enemy_neighbor_bundle;
use crate::components::Exposed;
use crate::components::TriggerRemaining;
use crate::components::{
    Coordinates, Damage, Enemy, EnemyNeighbor, GoldCoin, Grass, Health, OutWay, Player, Treasure,
    Uncover,
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

pub fn taggle_consumer(
    event: On<ToggleEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_options: Res<BoardOption>,
    mut effect_counters: ResMut<EffectCounters>,
    mut effect_phase_writer: MessageWriter<EffectPhaseMessage>,
    tile_type: Query<(
        Option<&Enemy>,
        Option<&EnemyNeighbor>,
        Option<&Grass>,
        Option<&Treasure>,
        Option<&OutWay>,
    )>,
    mut trigger_remaining: Query<&mut TriggerRemaining>,
    mut enemy_health_rw: ParamSet<(Query<&mut Health, With<Enemy>>, Query<&Health, With<Enemy>>)>,
    player_entity: Single<Entity, With<Player>>,
    player_damage: Query<&Damage, With<Player>>,
    mut player_gold: Query<&mut GoldCoin, With<Player>>,
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

    let (enemy, enemy_neighbor, grass, treasure, out_way) = match tile_type.get(tile_ent) {
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
    // 玩家攻击敌方格（须在效果广播之前结算，避免与其它 System 对 [`Health`] 的写入顺序混淆）
    // ---------------------------------------------------------------------------
    if enemy.is_some() {
        let coord = event.0;
        if let Ok(player_atk) = player_damage.get(*player_entity) {
            let killed_opt = match enemy_health_rw.p0().get_mut(tile_ent) {
                Ok(mut enemy_hp) => {
                    apply_player_damage_to_enemy_health(&mut enemy_hp, player_atk.0);
                    #[cfg(feature = "debug")]
                    log::info!(
                        "player attacks enemy: atk={}, enemy hp={}",
                        player_atk.0,
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
                    board.adjacent_enemy_hp_sum_from_entities(coord, &enemy_health_rw.p1());
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
                            board_options.padding,
                            board_size,
                            hp_sum_neighborhood,
                            &board_options.counter_font,
                        ))
                        .id();
                    commands.entity(new_ent).insert(ChildOf(board_parent));

                    board.tiles.insert(coord, new_ent);

                    if let Ok(mut gold) = player_gold.get_mut(*player_entity) {
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
                    &enemy_health_rw.p1(),
                    &children_q,
                    &mut text2d_q,
                );
            }
        } else {
            log::error!("player has no Damage component");
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

    if treasure.is_some() {
        log::info!("you get item");
    }

    if out_way.is_some() {
        log::info!("you get out way");
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
