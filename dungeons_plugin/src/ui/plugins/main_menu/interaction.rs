use bevy::{color::palettes::tailwind, prelude::*};

use crate::ui::plugins::main_menu::components::{StartGameButton, QuitButton};
use crate::AppState;
use crate::resources::board::Board;

pub fn interact_with_start_game_button(
    mut start_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (interaction, mut background_color) = match start_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            next_state.set(AppState::PreGame);
        }
        Interaction::Hovered => {
            background_color.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            background_color.0 = tailwind::SLATE_500.into();
        }
    }
}

pub fn interact_with_quit_button(
    mut quit_button: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut exit_msg_writer: MessageWriter<AppExit>
) {
    let (interaction, mut background_color) = match quit_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            exit_msg_writer.write(AppExit::Success);
        }
        Interaction::Hovered => {
            background_color.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            background_color.0 = tailwind::SLATE_500.into();
        }
    }
}