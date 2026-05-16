mod global;
mod io;
mod run;
mod snapshot;

pub use global::{
    load_global_save, merge_run_gems_into_global, persist_global_save, spawn_global_profile,
    GlobalProfile, GlobalSave, GLOBAL_SAVE_VERSION,
};
pub use io::SavePaths;
pub use run::{
    capture_run_save, delete_run_save, load_run_save, refresh_run_save_available, write_run_save,
    PendingRunRestore, RunSave, RunSaveAvailable, RUN_SAVE_VERSION,
};
pub use snapshot::RunPauseKind;
pub use snapshot::{
    apply_board_option_from_snapshot, apply_board_restoration, apply_player_snapshot,
    app_state_from_pause_kind, board_snapshot_from_board, restore_view, tile_map_from_snapshot,
    BoardSnapshot, PlayerSnapshot,
};

use bevy::prelude::*;

use crate::components::{Damage, Defense, Enemy, Gem, GoldCoin, Health, Player};
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::StageConfig;
use crate::resources::view2d::View2d;
use crate::AppState;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SavePaths>()
            .init_resource::<PendingRunRestore>()
            .init_resource::<RunSaveAvailable>()
            .add_systems(Startup, (setup_save_paths, init_global_profile).chain())
            .add_systems(OnEnter(AppState::GameOver), on_enter_game_over);
    }
}

fn setup_save_paths(mut paths: ResMut<SavePaths>) {
    *paths = SavePaths::new();
    paths.ensure_dir();
}

fn init_global_profile(mut commands: Commands, paths: Res<SavePaths>) {
    let save = load_global_save(paths.as_ref());
    spawn_global_profile(&mut commands, save.gems);
}

pub(crate) fn refresh_run_save_available_on_main_menu(
    paths: Res<SavePaths>,
    mut available: ResMut<RunSaveAvailable>,
) {
    refresh_run_save_available(paths.as_ref(), &mut available);
}

fn on_enter_game_over(paths: Res<SavePaths>, mut available: ResMut<RunSaveAvailable>) {
    delete_run_save(paths.as_ref(), &mut available);
}

/// 从局内返回主菜单时写档（须在 despawn Board 之前调用）。
pub fn save_run_before_board_teardown(
    paths: &SavePaths,
    available: &mut RunSaveAvailable,
    stage: &StageConfig,
    board: &Board,
    board_options: &BoardOption,
    enemy_health: &Query<&Health, With<Enemy>>,
    player: (&Health, &Damage, &Defense, &GoldCoin, &Gem),
    view2d: &View2d,
    paused_at: AppState,
) {
    capture_run_save(
        paths,
        available,
        stage,
        board,
        board_options,
        enemy_health,
        player,
        view2d,
        &paused_at,
    );
}
