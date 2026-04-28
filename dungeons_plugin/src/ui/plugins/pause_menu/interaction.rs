use bevy::{color::palettes::tailwind, prelude::*};

use crate::ui::plugins::pause_menu::components::{ResumeButton, RestartButton, QuitMainMenuButton};
use crate::AppState;
use crate::resources::board::Board;

pub fn interact_with_restart_button(
    mut restart_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (interaction, mut background_color) = match restart_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            next_state.set(AppState::RestartGame);
        }
        Interaction::Hovered => {
            background_color.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            background_color.0 = tailwind::SLATE_500.into();
        }
    }
}

pub fn interact_with_resume_button(
        mut resume_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ResumeButton>)>,
        mut next_state: ResMut<NextState<AppState>>
) {
    let (interaction, mut background_color) = match resume_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            next_state.set(AppState::InGame);
        }
        Interaction::Hovered => {
            background_color.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            background_color.0 = tailwind::SLATE_500.into();
        }
    }
}

pub fn interact_with_quit_main_menu_button(
    mut quit_main_menu_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitMainMenuButton>)>,
    mut next_state: ResMut<NextState<AppState>>
) {
    let (interaction, mut background_color) = match quit_main_menu_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            next_state.set(AppState::MainMenu);
        }
        Interaction::Hovered => {
            background_color.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            background_color.0 = tailwind::SLATE_500.into();
        }
    }
}