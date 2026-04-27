use bevy::prelude::*;

use crate::resources::board::Board;
use crate::components::view::View;

use crate::{
    AppState, 
    ui::{plugins::main_menu::{
        interacton::{
            interact_with_start_game_button, 
            interact_with_quit_button
        }, 
        layout::{
            despawn_main_menu, 
            spawn_main_menu
        }
    }}};

mod components;
mod interacton;
mod layout;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu), 
            spawn_main_menu
        )
        .add_systems(
            OnExit(AppState::MainMenu), 
            despawn_main_menu
        )
        .add_systems(
            Update, 
            (
                interact_with_start_game_button,
                interact_with_quit_button
            ).run_if(in_state(AppState::MainMenu))
        );
    }
}

impl MainMenuPlugin {
    fn clean_board(
        mut commands: Commands,
        board: Res<Board>,
        view: Single<&mut Transform, With<View>>,
    ) {
        if board.board_entity.is_some() {
            commands.entity(board.board_entity.unwrap()).despawn();
        }
        commands.remove_resource::<Board>();
        let mut transform = view.into_inner();
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
}