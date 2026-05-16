use bevy::prelude::*;

use crate::{
    save::refresh_run_save_available_on_main_menu,
    AppState,
    ui::plugins::main_menu::{
        character_interaction::{
            interact_with_character_portrait, keyboard_switch_main_menu_character,
            refresh_main_menu_character_display,
        },
        interaction::{
            interact_with_continue_run_button, interact_with_quit_button,
            interact_with_start_game_button,
        },
        layout::{despawn_main_menu, spawn_main_menu},
    },
};

mod character_interaction;
mod components;
mod interaction;
mod layout;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (
                refresh_run_save_available_on_main_menu,
                spawn_main_menu,
            )
                .chain(),
        )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (
                    keyboard_switch_main_menu_character,
                    interact_with_character_portrait,
                    refresh_main_menu_character_display,
                    interact_with_start_game_button,
                    interact_with_continue_run_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
