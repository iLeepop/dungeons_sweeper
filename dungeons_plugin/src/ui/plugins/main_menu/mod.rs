use bevy::prelude::*;

use crate::{
    AppState,
    ui::plugins::main_menu::{
        interaction::{interact_with_quit_button, interact_with_start_game_button},
        layout::{despawn_main_menu, spawn_main_menu},
    },
};

mod components;
mod interaction;
mod layout;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (interact_with_start_game_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
