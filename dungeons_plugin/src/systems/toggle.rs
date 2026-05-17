use bevy::log;
use bevy::prelude::*;

use crate::components::{Coordinates, Enemy, EnemyNeighbor, Grass, OutWay, Treasure, Uncover};

pub fn uncover_tile(
    mut commands: Commands,
    children: Query<(Entity, &ChildOf), With<Uncover>>,
    query: Query<(
        &Coordinates,
        Option<&Enemy>,
        Option<&EnemyNeighbor>,
        Option<&Grass>,
        Option<&Treasure>,
        Option<&OutWay>,
    )>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn();

        let (_coordinates, enemy, enemy_neighbor, grass, treasure, out_way) =
            match query.get(parent.0) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Error getting tile: {:?}", e);
                    continue;
                }
            };

        if enemy.is_some() {}

        if enemy_neighbor.is_some() {}

        if grass.is_some() {}

        if treasure.is_some() {}

        if out_way.is_some() {}
    }
}
