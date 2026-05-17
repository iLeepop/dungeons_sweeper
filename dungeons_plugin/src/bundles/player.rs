use bevy::prelude::*;

use crate::character::{CharacterId, RunCharacter};
use crate::components::{Damage, Defense, Gem, GoldCoin, Health, Player};
use crate::effects::{ActiveEffectSpecs, SerializableEffect, build_player_loader};
use crate::resources::PlayerOptions;
use crate::save::PlayerSnapshot;

/// 按 [`PlayerOptions`] 组装玩家实体（数值单一数据源，避免与 HUD/关卡公式分叉）。
pub fn player_bundle(
    opts: &PlayerOptions,
    character_id: CharacterId,
    effect_specs: &[SerializableEffect],
) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        RunCharacter(character_id),
        ActiveEffectSpecs(effect_specs.to_vec()),
        GoldCoin(0),
        Gem(0),
        Health(opts.starting_hp),
        Damage(opts.starting_damage),
        Defense(opts.starting_defense),
        build_player_loader(effect_specs),
    )
}

/// 从局内存档恢复玩家（避免 spawn 当帧 Query 拿不到新实体）。
pub fn player_bundle_from_snapshot(
    _opts: &PlayerOptions,
    snap: &PlayerSnapshot,
    character_id: CharacterId,
) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        RunCharacter(character_id),
        ActiveEffectSpecs(snap.effect_specs.clone()),
        GoldCoin(snap.gold),
        Gem(snap.gems),
        Health(snap.health),
        Damage(snap.damage),
        Defense(snap.defense),
        build_player_loader(&snap.effect_specs),
    )
}
