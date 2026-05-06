use bevy::ecs::observer::On;
use bevy::log;
use bevy::prelude::*;

use crate::components::Exposed;
use crate::components::{
    Damage, Defense, Enemy, EnemyNeighbor, Grass, Health, Treasure, OutWay, Uncover,
};
use crate::events::taggle::ToggleEvent;
use crate::resources::board::Board;

pub fn taggle_consumer(
    event: On<ToggleEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
    tile_type: Query<(
        Option<&Enemy>,
        Option<&EnemyNeighbor>,
        Option<&Grass>,
        Option<&Treasure>,
        Option<&OutWay>,
    )>,
    status: Query<(Option<&Health>, Option<&Damage>, Option<&Defense>)>,
    parent: Query<&ChildOf>,
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
        return;
    }
    if let Some(tile) = board.tiles.get(&event.0) {
        #[cfg(feature = "debug")]
        log::info!("despawn tile: {:?}", *tile);

        let (enemy, enemy_neighbor, grass, treasure, out_way) = match tile_type.get(*tile) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error getting tile: {:?}", e);
                return;
            }
        };

        if enemy.is_some() {
            // 获取属性组件
            let (_health, damage, _defense) = match status.get(*tile) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Error getting status: {:?}", e);
                    return;
                }
            };
            let damage = damage.unwrap().0;
            log::info!("you get hurt by enemy {} damage", damage);
        }

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
    }
}
