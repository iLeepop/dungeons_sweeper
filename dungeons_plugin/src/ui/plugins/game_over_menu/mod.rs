use bevy::prelude::*;

mod components;
mod interaction;
mod layout;

use crate::AppState;
use crate::ui::plugins::game_over_menu::interaction::{
    interact_with_game_over_quit_main_menu, interact_with_game_over_restart,
};
use crate::ui::plugins::game_over_menu::layout::{despawn_game_over_menu, spawn_game_over_menu};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu)
            .add_systems(
                Update,
                (
                    interact_with_game_over_restart,
                    interact_with_game_over_quit_main_menu,
                )
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
