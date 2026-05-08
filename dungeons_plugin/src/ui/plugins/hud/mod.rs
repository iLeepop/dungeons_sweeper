use bevy::prelude::*;

mod components;
mod interaction;
mod layout;

pub use components::*;
pub use interaction::*;
pub use layout::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                exited: AppState::PreGame,
                entered: AppState::InGame,
            }, 
            spawn_hud
        )
            .add_systems(
                OnTransition {
                    exited: AppState::GamePause,
                    entered: AppState::MainMenu,
                },
                despawn_hud,
            )
            .add_systems(
                OnTransition {
                    exited: AppState::GamePause,
                    entered: AppState::PreGame,
                },
                despawn_hud,
            );
    }
}
