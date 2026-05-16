use bevy::{color::palettes::tailwind, prelude::*};

use crate::character::{
    effects_from_character, persist_character_selection, PendingNewRunSetup, SelectedCharacter,
    UnlockedCharacters,
};
use crate::components::Gem;
use crate::save::GlobalProfile;
use crate::AppState;
use crate::components::Player;
use crate::resources::StageConfig;
use crate::save::{delete_run_save, load_run_save, PendingRunRestore, RunSaveAvailable, SavePaths};
use crate::ui::plugins::main_menu::components::{
    ContinueRunButton, QuitButton, StartGameButton,
};

pub fn interact_with_start_game_button(
    mut start_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut stage: ResMut<StageConfig>,
    paths: Res<SavePaths>,
    mut run_available: ResMut<RunSaveAvailable>,
    mut pending: ResMut<PendingRunRestore>,
    mut new_run: ResMut<PendingNewRunSetup>,
    selected: Res<SelectedCharacter>,
    unlocked: Res<UnlockedCharacters>,
    global_gem: Single<&Gem, With<GlobalProfile>>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    if !unlocked.is_unlocked(selected.id) {
        return;
    }

    let (interaction, mut background_color) = match start_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            persist_character_selection(
                paths.as_ref(),
                global_gem.0,
                unlocked.as_ref(),
                selected.id,
            );
            let specs = effects_from_character(selected.id);
            new_run.character_id = Some(selected.id);
            new_run.effect_specs = specs;
            pending.0 = None;
            delete_run_save(paths.as_ref(), &mut run_available);
            for entity in players.iter() {
                commands.entity(entity).despawn();
            }
            stage.reset_to_first_stage();
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

pub fn interact_with_continue_run_button(
    mut continue_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ContinueRunButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    paths: Res<SavePaths>,
    mut pending: ResMut<PendingRunRestore>,
    run_available: Res<RunSaveAvailable>,
    selected: Res<SelectedCharacter>,
    unlocked: Res<UnlockedCharacters>,
) {
    if !run_available.0 || !unlocked.is_unlocked(selected.id) {
        return;
    }

    let (interaction, mut background_color) = match continue_button.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    match interaction {
        Interaction::Pressed => {
            background_color.0 = tailwind::SLATE_700.into();
            if let Some(save) = load_run_save(paths.as_ref()) {
                pending.0 = Some(save);
                next_state.set(AppState::PreGame);
            }
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
    mut quit_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit_msg_writer: MessageWriter<AppExit>,
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
