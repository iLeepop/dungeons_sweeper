use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::Gem;
use crate::save::io::{read_ron, write_ron, SavePaths};

pub const GLOBAL_SAVE_VERSION: u32 = 1;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GlobalProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSave {
    pub version: u32,
    pub gems: u32,
}

impl Default for GlobalSave {
    fn default() -> Self {
        Self {
            version: GLOBAL_SAVE_VERSION,
            gems: 0,
        }
    }
}

pub fn load_global_save(paths: &SavePaths) -> GlobalSave {
    read_ron::<GlobalSave>(&paths.global)
        .filter(|s| s.version == GLOBAL_SAVE_VERSION)
        .unwrap_or_default()
}

pub fn persist_global_save(paths: &SavePaths, gems: u32) {
    paths.ensure_dir();
    let save = GlobalSave {
        version: GLOBAL_SAVE_VERSION,
        gems,
    };
    if !write_ron(&paths.global, &save) {
        bevy::log::error!("failed to write global save");
    }
}

/// 将局内宝石累加到全局实体并写盘；返回合并后的全局宝石总数。
pub fn merge_run_gems_into_global(
    paths: &SavePaths,
    global_gem: &mut Gem,
    run_gems: u32,
) -> u32 {
    global_gem.0 = global_gem.0.saturating_add(run_gems);
    persist_global_save(paths, global_gem.0);
    global_gem.0
}

pub fn spawn_global_profile(commands: &mut Commands, gems: u32) {
    commands.spawn((
        Name::new("GlobalProfile"),
        GlobalProfile,
        Gem(gems),
    ));
}
