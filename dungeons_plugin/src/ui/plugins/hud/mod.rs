use bevy::prelude::*;

mod layout;
mod interaction;
mod components;

pub use layout::*;
pub use interaction::*;
pub use components::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(AppState::InGame), 
            spawn_hud
        )
        .add_systems(
            OnTransition {
                exited: AppState::GamePause,
                entered: AppState::MainMenu
            },
            despawn_hud
        );
    }
}