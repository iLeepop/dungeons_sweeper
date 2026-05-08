use bevy::log;
use bevy::prelude::*;

use crate::components::{Damage, Defense, Health, Player};
use crate::events::player::PlayerHurt;

pub fn player_action(
    event: On<PlayerHurt>,
    player: Single<Entity, With<Player>>,
    mut status: Query<(
        Option<&mut Health>,
        Option<&mut Damage>,
        Option<&mut Defense>,
    )>,
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
                // HUD 由 `sync_player_hud_from_components` 在 PostUpdate 末从组件统一刷新
                if health.0 <= 0 {
                    #[cfg(feature = "debug")]
                    log::info!("player is dead");
                }
            }
        }
    }
}
