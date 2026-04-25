use bevy::prelude::*;
use bevy::log;

use crate::components::{Player, Health, Damage, Defense};
use crate::events::player::PlayerHurt;

pub fn player_action(
    event: On<PlayerHurt>,
    player: Single<Entity, With<Player>>,
    mut status: Query<(Option<&mut Health>, Option<&mut Damage>, Option<&mut Defense>)>
) {
    #[cfg(feature = "debug")]
    log::info!("player action");
    #[cfg(feature = "debug")]
    log::info!("player action: {:?}", *player);
    let (health, _damage, defense) = match status.get_mut(*player) {
        Ok(v) => v,
        Err(_) => return,
    };
    if let Some(mut defense) = defense {
        #[cfg(feature = "debug")]
        log::info!("player get hurt: {}", event.0);
        defense.0 -= std::cmp::min(defense.0, event.0.try_into().unwrap_or(0));
        if defense.0 <= 0 {
            if let Some(mut health) = health {
                health.0 -= std::cmp::min(health.0, event.0.try_into().unwrap_or(0));
                if health.0 <= 0 {
                    println!("player is dead");
                }
            }
        }
    }
}