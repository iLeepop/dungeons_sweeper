use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{Damage, Defense, Enemy, Gem, GoldCoin, Health, Player};
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::StageConfig;
use crate::resources::view2d::View2d;
use crate::save::io::{delete_file, file_exists, read_ron, write_ron, SavePaths};
use crate::save::snapshot::{
    board_snapshot_from_board, capture_player_snapshot, pause_kind_from_state, BoardSnapshot,
    PlayerSnapshot, };
use crate::save::snapshot::RunPauseKind;
use crate::AppState;

pub const RUN_SAVE_VERSION: u32 = 1;

#[derive(Resource, Default)]
pub struct PendingRunRestore(pub Option<RunSave>);

#[derive(Resource, Default)]
pub struct RunSaveAvailable(pub bool);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSave {
    pub version: u32,
    pub stage: u32,
    pub board: BoardSnapshot,
    pub player: PlayerSnapshot,
    pub view: Option<[f32; 3]>,
    pub paused_at: RunPauseKind,
}

pub fn refresh_run_save_available(paths: &SavePaths, available: &mut RunSaveAvailable) {
    available.0 = file_exists(&paths.run);
}

pub fn load_run_save(paths: &SavePaths) -> Option<RunSave> {
    let save: RunSave = read_ron(&paths.run)?;
    if save.version != RUN_SAVE_VERSION {
        bevy::log::warn!(
            "run save version mismatch: got {}, want {}",
            save.version,
            RUN_SAVE_VERSION
        );
        delete_file(&paths.run);
        return None;
    }
    Some(save)
}

pub fn write_run_save(paths: &SavePaths, save: &RunSave) {
    paths.ensure_dir();
    if !write_ron(&paths.run, save) {
        bevy::log::error!("failed to write run save");
    }
}

pub fn delete_run_save(paths: &SavePaths, available: &mut RunSaveAvailable) {
    delete_file(&paths.run);
    available.0 = false;
}

pub fn capture_run_save(
    paths: &SavePaths,
    available: &mut RunSaveAvailable,
    stage: &StageConfig,
    board: &Board,
    board_options: &BoardOption,
    enemy_health: &Query<&Health, With<Enemy>>,
    player: (
        &Health,
        &Damage,
        &Defense,
        &GoldCoin,
        &Gem,
    ),
    view2d: &View2d,
    exited_state: &AppState,
) {
    let board_snap = board_snapshot_from_board(board, board_options, enemy_health);
    let player_snap = capture_player_snapshot(player.0, player.1, player.2, player.3, player.4);
    let view = Some([view2d.position.x, view2d.position.y, view2d.position.z]);
    let save = RunSave {
        version: RUN_SAVE_VERSION,
        stage: stage.stage,
        board: board_snap,
        player: player_snap,
        view,
        paused_at: pause_kind_from_state(exited_state),
    };
    write_run_save(paths, &save);
    available.0 = true;
}
