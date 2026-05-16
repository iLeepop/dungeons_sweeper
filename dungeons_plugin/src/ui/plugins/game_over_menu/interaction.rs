use bevy::{color::palettes::tailwind, prelude::*};

use crate::AppState;
use crate::components::Player;
use crate::save::{delete_run_save, RunSaveAvailable, SavePaths};
use crate::components::view::View;
use crate::resources::View2d;
use crate::resources::board::Board;
use crate::resources::StageConfig;
use crate::ui::plugins::game_over_menu::components::{
    GameOverQuitMainMenuButton, GameOverRestartButton,
};

pub fn interact_with_game_over_restart(
    mut btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GameOverRestartButton>),
    >,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut stage: ResMut<StageConfig>,
    board: Res<Board>,
    player: Single<Entity, With<Player>>,
    paths: Res<SavePaths>,
    mut run_available: ResMut<RunSaveAvailable>,
) {
    let (interaction, mut bg) = match btn.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };
    match interaction {
        Interaction::Pressed => {
            bg.0 = tailwind::SLATE_700.into();
            delete_run_save(paths.as_ref(), &mut run_available);
            // 与主菜单「开始游戏」一致：整局重来须回到第 1 关（地图尺寸与难度系数）。
            stage.reset_to_first_stage();
            if board.board_entity.is_some() {
                commands.entity(board.board_entity.unwrap()).despawn();
            }
            commands.remove_resource::<Board>();
            commands.entity(*player).despawn();
            next_state.set(AppState::PreGame);
        }
        Interaction::Hovered => {
            bg.0 = tailwind::SLATE_600.into();
        }
        Interaction::None => {
            bg.0 = tailwind::SLATE_500.into();
        }
    }
}

pub fn interact_with_game_over_quit_main_menu(
    mut btn: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GameOverQuitMainMenuButton>),
    >,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    board: Res<Board>,
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
