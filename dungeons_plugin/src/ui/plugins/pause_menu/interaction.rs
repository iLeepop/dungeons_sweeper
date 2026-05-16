use bevy::{color::palettes::tailwind, prelude::*};

use crate::AppState;
use crate::character::RunCharacter;
use crate::components::{Damage, Defense, Enemy, Gem, GoldCoin, Health, Player};
use crate::effects::ActiveEffectSpecs;
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::StageConfig;
use crate::resources::View2d;
use crate::save::{
    delete_run_save, save_run_before_board_teardown, RunSaveAvailable, SavePaths,
};
use crate::ui::plugins::pause_menu::components::{QuitMainMenuButton, RestartButton, ResumeButton};

pub fn interact_with_restart_button(
    mut restart_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    paths: Res<SavePaths>,
    mut run_available: ResMut<RunSaveAvailable>,
) {
    let (interaction, mut background_color) = match restart_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            delete_run_save(paths.as_ref(), &mut run_available);
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

pub fn interact_with_resume_button(
    mut resume_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
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
    mut quit_main_menu_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitMainMenuButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    board: Res<Board>,
    board_options: Res<BoardOption>,
    stage: Res<StageConfig>,
    enemy_health: Query<&Health, With<Enemy>>,
    player: Single<
        (
            &Health,
            &Damage,
            &Defense,
            &GoldCoin,
            &Gem,
            &RunCharacter,
            &ActiveEffectSpecs,
        ),
        With<Player>,
    >,
    paths: Res<SavePaths>,
    mut run_available: ResMut<RunSaveAvailable>,
    view2d: Res<View2d>,
) {
    let (interaction, mut background_color) = match quit_main_menu_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            save_run_before_board_teardown(
                paths.as_ref(),
                &mut run_available,
                stage.as_ref(),
                board.as_ref(),
                board_options.as_ref(),
                &enemy_health,
                (
                    &player.0,
                    &player.1,
                    &player.2,
                    &player.3,
                    &player.4,
                ),
                &player.5,
                &player.6,
                view2d.as_ref(),
                AppState::GamePause,
            );
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
