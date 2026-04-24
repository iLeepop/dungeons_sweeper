use bevy::prelude::*;
use bevy::log;

use crate::components::{Exposed, Enemy, Health, Damage, Defense};
use crate::events::enemy_event::EnemyAttackEvent;
use crate::events::player::PlayerHurt;

pub fn enemy_havier_handler(
    _event: On<EnemyAttackEvent>,
    mut commands: Commands,
    uncover_enemy: Query<(Entity, Option<&Enemy>), With<Exposed>>,
    status: Query<(Option<&Health>, Option<&Damage>, Option<&Defense>)>
) {
    #[cfg(feature = "debug")]
    log::info!("enemy havier handler");
    for (entity, enemy) in uncover_enemy.iter() {
        #[cfg(feature = "debug")]
        log::info!("enemy havier handler: {:?}", enemy);
        if enemy.is_some() {
            let (health, damage, _defense) = match status.get(entity) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(health) = health {
                if health.0 == 0 {
                    commands.entity(entity).despawn();
                }
            }

            if let Some(damage) = damage {
                #[cfg(feature = "debug")]
                log::info!("player hurt: {:?}", damage.0);
                commands.trigger(PlayerHurt(damage.0));
            }
        }
    }
}