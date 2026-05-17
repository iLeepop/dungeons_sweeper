use bevy::prelude::*;

use crate::character::id::CharacterId;
use crate::save::{GlobalSave, SavePaths, persist_global_save};

#[derive(Resource, Debug, Clone, Default)]
pub struct SelectedCharacter {
    pub id: CharacterId,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct UnlockedCharacters {
    pub ids: Vec<CharacterId>,
}

impl UnlockedCharacters {
    pub fn from_save(save: &GlobalSave) -> Self {
        let ids: Vec<CharacterId> = save
            .unlocked_characters
            .iter()
            .filter_map(|&i| CharacterId::from_index(i))
            .collect();
        Self {
            ids: if ids.is_empty() {
                vec![CharacterId::Herbalist]
            } else {
                ids
            },
        }
    }

    pub fn to_save_indices(&self) -> Vec<u8> {
        self.ids.iter().map(|id| id.to_index()).collect()
    }

    pub fn is_unlocked(&self, id: CharacterId) -> bool {
        self.ids.contains(&id)
    }

    pub fn unlock(&mut self, id: CharacterId) {
        if !self.ids.contains(&id) {
            self.ids.push(id);
        }
    }
}

pub fn try_unlock_character(
    paths: &SavePaths,
    global_gems: &mut crate::components::Gem,
    unlocked: &mut UnlockedCharacters,
    selected: &mut SelectedCharacter,
    id: CharacterId,
) -> bool {
    if unlocked.is_unlocked(id) {
        selected.id = id;
        return true;
    }
    let cost = crate::character::defs::character_def(id)
        .unlock_cost
        .unwrap_or(0);
    if global_gems.0 < cost {
        return false;
    }
    global_gems.0 -= cost;
    unlocked.unlock(id);
    selected.id = id;
    persist_global_save(
        paths,
        global_gems.0,
        &unlocked.to_save_indices(),
        selected.id.to_index(),
    );
    true
}

pub fn persist_character_selection(
    paths: &SavePaths,
    global_gems: u32,
    unlocked: &UnlockedCharacters,
    selected: CharacterId,
) {
    persist_global_save(
        paths,
        global_gems,
        &unlocked.to_save_indices(),
        selected.to_index(),
    );
}
