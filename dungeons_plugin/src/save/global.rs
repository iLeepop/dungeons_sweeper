use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::CharacterId;
use crate::character::{SelectedCharacter, UnlockedCharacters};
use crate::components::Gem;
use crate::save::io::{SavePaths, read_ron, write_ron};

pub const GLOBAL_SAVE_VERSION: u32 = 2;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GlobalProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSave {
    pub version: u32,
    pub gems: u32,
    #[serde(default = "default_unlocked")]
    pub unlocked_characters: Vec<u8>,
    #[serde(default)]
    pub last_selected_character: u8,
}

fn default_unlocked() -> Vec<u8> {
    vec![CharacterId::Herbalist.to_index()]
}

impl Default for GlobalSave {
    fn default() -> Self {
        Self {
            version: GLOBAL_SAVE_VERSION,
            gems: 0,
            unlocked_characters: default_unlocked(),
            last_selected_character: CharacterId::Herbalist.to_index(),
        }
    }
}

pub fn load_global_save(paths: &SavePaths) -> GlobalSave {
    let Some(mut save) = read_ron::<GlobalSave>(&paths.global) else {
        return GlobalSave::default();
    };
    if save.version == 1 {
        return GlobalSave {
            version: GLOBAL_SAVE_VERSION,
            gems: save.gems,
            unlocked_characters: default_unlocked(),
            last_selected_character: CharacterId::Herbalist.to_index(),
        };
    }
    if save.version != GLOBAL_SAVE_VERSION {
        return GlobalSave::default();
    }
    if save.unlocked_characters.is_empty() {
        save.unlocked_characters = default_unlocked();
    }
    save
}

pub fn persist_global_save(
    paths: &SavePaths,
    gems: u32,
    unlocked_characters: &[u8],
    last_selected_character: u8,
) {
    paths.ensure_dir();
    let save = GlobalSave {
        version: GLOBAL_SAVE_VERSION,
        gems,
        unlocked_characters: unlocked_characters.to_vec(),
        last_selected_character,
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
    unlocked: &UnlockedCharacters,
    selected: CharacterId,
) -> u32 {
    global_gem.0 = global_gem.0.saturating_add(run_gems);
    persist_global_save(
        paths,
        global_gem.0,
        &unlocked.to_save_indices(),
        selected.to_index(),
    );
    global_gem.0
}

pub fn award_stage_gems_to_global(
    paths: &SavePaths,
    global_gem: &mut Gem,
    stage: u32,
    unlocked: &UnlockedCharacters,
    selected: CharacterId,
) -> u32 {
    let earned = stage.max(1);
    global_gem.0 = global_gem.0.saturating_add(earned);
    persist_global_save(
        paths,
        global_gem.0,
        &unlocked.to_save_indices(),
        selected.to_index(),
    );
    earned
}

pub fn spawn_global_profile(commands: &mut Commands, save: &GlobalSave) {
    commands.spawn((Name::new("GlobalProfile"), GlobalProfile, Gem(save.gems)));
}

pub fn init_character_resources_from_save(commands: &mut Commands, save: &GlobalSave) {
    let unlocked = UnlockedCharacters::from_save(save);
    let selected = CharacterId::from_index(save.last_selected_character)
        .filter(|id| unlocked.is_unlocked(*id))
        .unwrap_or(CharacterId::Herbalist);
    commands.insert_resource(SelectedCharacter { id: selected });
    commands.insert_resource(unlocked);
}
