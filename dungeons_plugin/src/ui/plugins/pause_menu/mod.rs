use bevy::prelude::*;

mod components;
mod interaction;
mod layout;

use crate::AppState;
use crate::components::view::View;
use crate::resources::View2d;
use crate::resources::board::Board;
use crate::ui::plugins::pause_menu::interaction::{
    interact_with_quit_main_menu_button, interact_with_restart_button, interact_with_resume_button,
};
use crate::ui::plugins::pause_menu::layout::{despawn_pause_menu, spawn_pause_menu};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::pause_game.run_if(in_state(AppState::InGame).or(in_state(AppState::GamePause))),
        )
        .add_systems(OnEnter(AppState::GamePause), spawn_pause_menu)
        .add_systems(OnExit(AppState::GamePause), despawn_pause_menu)
        .add_systems(
            Update,
            (
                interact_with_restart_button,
                interact_with_resume_button,
                interact_with_quit_main_menu_button,
            )
                .run_if(in_state(AppState::GamePause)),
        )
        .add_systems(OnEnter(AppState::RestartGame), Self::restart_game)
        .add_systems(
            OnTransition {
                exited: AppState::GamePause,
                entered: AppState::MainMenu,
            },
            Self::exit_game,
        );
    }
}

impl PauseMenuPlugin {
    fn pause_game(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        state: Res<State<AppState>>,
        mut next_state: ResMut<NextState<AppState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            if *state == AppState::InGame {
                next_state.set(AppState::GamePause);
            } else {
                next_state.set(AppState::InGame);
            }
        }
    }

    fn restart_game(
        mut commands: Commands,
        mut next_state: ResMut<NextState<AppState>>,
        board: Res<Board>,
    ) {
        if board.board_entity.is_some() {
            commands.entity(board.board_entity.unwrap()).despawn();
        }
        commands.remove_resource::<Board>();
        next_state.set(AppState::PreGame);
    }

    fn exit_game(
        mut commands: Commands,
        board: Res<Board>,
        mut view2d: ResMut<View2d>,
        view: Single<&mut Transform, With<View>>,
        // mut next_state: ResMut<NextState<AppState>>,
    ) {
        if board.board_entity.is_some() {
            commands.entity(board.board_entity.unwrap()).despawn();
        }
        commands.remove_resource::<Board>();
        let mut transform = view.into_inner();
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
        view2d.position = Vec3::new(0.0, 0.0, 0.0);
        // next_state.set(AppState::MainMenu);
    }
}
