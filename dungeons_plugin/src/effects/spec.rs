//! 可序列化的效果规格：用于角色定义、存档与运行时重建加载器。

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::effects::builtin::{GrassHealPlayer, KillBonusDamage};
use crate::effects::entry::EffectEntry;
use crate::effects::loaders::PlayerEffectLoader;
use crate::effects::trigger::{EffectPhase, EffectTrigger};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SerializableEffect {
    GrassHealOnTile { amount: i8 },
    KillBonusDamage { amount: u8 },
}

/// 本局玩家挂载的效果列表（写档时直接读取）。
#[derive(Component, Debug, Clone, Default)]
pub struct ActiveEffectSpecs(pub Vec<SerializableEffect>);

impl SerializableEffect {
    pub fn to_player_entry(&self) -> Option<EffectEntry> {
        match self {
            SerializableEffect::KillBonusDamage { amount } => Some(EffectEntry::new(
                0,
                EffectTrigger::OnPhase(EffectPhase::AfterEnemyKill),
                KillBonusDamage(*amount),
            )),
            SerializableEffect::GrassHealOnTile { .. } => None,
        }
    }
}

pub fn build_player_loader(specs: &[SerializableEffect]) -> PlayerEffectLoader {
    let mut loader = PlayerEffectLoader::default();
    for spec in specs {
        if let Some(entry) = spec.to_player_entry() {
            loader.push(entry);
        }
    }
    loader
}

pub fn grass_heal_amount_from_specs(specs: &[SerializableEffect]) -> Option<i8> {
    specs.iter().find_map(|s| match s {
        SerializableEffect::GrassHealOnTile { amount } => Some(*amount),
        _ => None,
    })
}

pub fn capture_effect_specs(active: &ActiveEffectSpecs) -> Vec<SerializableEffect> {
    active.0.clone()
}
