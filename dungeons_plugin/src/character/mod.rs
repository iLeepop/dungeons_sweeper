mod defs;
mod id;
mod unlock;

pub use defs::{ALL_CHARACTERS, character_def, effects_from_character};
pub use id::CharacterId;
pub use unlock::{
    SelectedCharacter, UnlockedCharacters, persist_character_selection, try_unlock_character,
};

use bevy::prelude::*;

use crate::character::id::CharacterId as CharId;
use crate::effects::SerializableEffect;

/// 新游戏进入 PreGame 前由主菜单写入；`create_board` 消费后清空。
#[derive(Resource, Default)]
pub struct PendingNewRunSetup {
    pub character_id: Option<CharId>,
    pub effect_specs: Vec<SerializableEffect>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct RunCharacter(pub CharId);
