use bevy::prelude::*;

use crate::character::id::CharacterId;
use crate::effects::SerializableEffect;

pub struct CharacterDef {
    pub id: CharacterId,
    pub display_name: &'static str,
    pub unlock_cost: Option<u32>,
    pub portrait_color: Color,
    pub effects: &'static [SerializableEffect],
}

const HERBALIST_EFFECTS: &[SerializableEffect] =
    &[SerializableEffect::GrassHealOnTile { amount: 1 }];

const SLAYER_EFFECTS: &[SerializableEffect] = &[SerializableEffect::KillBonusDamage { amount: 1 }];

pub const ALL_CHARACTERS: &[CharacterDef] = &[
    CharacterDef {
        id: CharacterId::Herbalist,
        display_name: "草药师",
        unlock_cost: None,
        portrait_color: Color::srgb(0.2, 0.85, 0.35),
        effects: HERBALIST_EFFECTS,
    },
    CharacterDef {
        id: CharacterId::Slayer,
        display_name: "猎杀者",
        unlock_cost: Some(10),
        portrait_color: Color::srgb(0.85, 0.2, 0.25),
        effects: SLAYER_EFFECTS,
    },
];

pub fn character_def(id: CharacterId) -> &'static CharacterDef {
    ALL_CHARACTERS
        .iter()
        .find(|c| c.id == id)
        .expect("unknown character id")
}

pub fn effects_from_character(id: CharacterId) -> Vec<SerializableEffect> {
    character_def(id).effects.to_vec()
}
