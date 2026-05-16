use bevy::{color::palettes::tailwind, prelude::*};

use crate::advance_stage_and_rebuild_board;
use crate::AppState;
use crate::components::view::View;
use crate::components::{Damage, Defense, Enemy, Gem, GoldCoin, Health, Player};
use crate::effects::WorldEffectHost;
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::DifficultyTuning;
use crate::resources::enemy_assets::EnemyAssets;
use crate::resources::PlayerOptions;
use crate::resources::StageConfig;
use crate::resources::View2d;
use crate::save::{
    merge_run_gems_into_global, save_run_before_board_teardown, GlobalProfile, RunSaveAvailable,
    SavePaths,
};
use crate::ui::plugins::next_level::components::{
    NextLevelContinueButton, NextLevelQuitMainMenuButton,
};

pub fn interact_with_next_level_continue(
    mut btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NextLevelContinueButton>),
    >,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut board_options: ResMut<BoardOption>,
    mut stage: ResMut<StageConfig>,
    enemy_assets: Res<EnemyAssets>,
    player_options: Res<PlayerOptions>,
    tuning: Res<DifficultyTuning>,
    board: Res<Board>,
    world_hosts: Query<Entity, With<WorldEffectHost>>,
    paths: Res<SavePaths>,
    mut global_gem: Single<&mut Gem, With<GlobalProfile>>,
    mut player_gem: Single<&mut Gem, (With<Player>, Without<GlobalProfile>)>,
) {
    let (interaction, mut bg) = match btn.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };
    match interaction {
        Interaction::Pressed => {
            bg.0 = tailwind::EMERALD_800.into();
            let run_gems = player_gem.0;
            player_gem.0 = 0;
            merge_run_gems_into_global(paths.as_ref(), &mut global_gem, run_gems);
            let board_ent = board.board_entity;
            advance_stage_and_rebuild_board(
                &mut commands,
                &mut *board_options,
                &mut *stage,
                enemy_assets.as_ref(),
                player_options.as_ref(),
                tuning.as_ref(),
                &world_hosts,
                board_ent,
            );
            next_state.set(AppState::InGame);
        }
        Interaction::Hovered => {
            bg.0 = tailwind::EMERALD_500.into();
        }
        Interaction::None => {
            bg.0 = tailwind::EMERALD_600.into();
        }
    }
}

pub fn interact_with_next_level_quit_main_menu(
    mut btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NextLevelQuitMainMenuButton>),
    >,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    board: Res<Board>,
    board_options: Res<BoardOption>,
    stage: Res<StageConfig>,
    enemy_health: Query<&Health, With<Enemy>>,
    player: Single<(&Health, &Damage, &Defense, &GoldCoin, &Gem), With<Player>>,
    paths: Res<SavePaths>,
    mut run_available: ResMut<RunSaveAvailable>,
    mut view2d: ResMut<View2d>,
    view: Single<&mut Transform, With<View>>,
) {
    let (interaction, mut bg) = match btn.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };
    match interaction {
        Interaction::Pressed => {
            bg.0 = tailwind::SLATE_700.into();
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
                view2d.as_ref(),
                AppState::NextLevel,
            );
            if board.board_entity.is_some() {
                commands.entity(board.board_entity.unwrap()).despawn();
            }
            commands.remove_resource::<Board>();
            let mut xf = view.into_inner();
            xf.translation = Vec3::new(0.0, 0.0, 0.0);
            view2d.position = Vec3::new(0.0, 0.0, 0.0);
            next_state.set(AppState::MainMenu);
        }
        Interaction::Hovered => {
            bg.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            bg.0 = tailwind::SLATE_500.into();
        }
    }
}
