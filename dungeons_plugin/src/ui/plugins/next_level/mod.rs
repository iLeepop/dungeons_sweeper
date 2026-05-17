use bevy::prelude::*;

mod components;
mod interaction;
mod layout;

use crate::AppState;
use crate::ui::plugins::next_level::interaction::{
    interact_with_next_level_continue, interact_with_next_level_quit_main_menu,
};
use crate::ui::plugins::next_level::layout::{despawn_next_level_menu, spawn_next_level_menu};

pub struct NextLevelPlugin;

impl Plugin for NextLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::NextLevel), spawn_next_level_menu)
            .add_systems(OnExit(AppState::NextLevel), despawn_next_level_menu)
            .add_systems(
                Update,
                (
                    interact_with_next_level_continue,
                    interact_with_next_level_quit_main_menu
                )
                    .run_if(in_state(AppState::NextLevel)),
            );
    }
}
