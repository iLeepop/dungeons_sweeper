use bevy::prelude::*;

use crate::components::{Damage, Defense, Health, Player, GoldCoin, Gem};
use crate::effects::PlayerEffectLoader;
use crate::resources::PlayerOptions;
use crate::save::PlayerSnapshot;

/// 按 [`PlayerOptions`] 组装玩家实体（数值单一数据源，避免与 HUD/关卡公式分叉）。
pub fn player_bundle(opts: &PlayerOptions) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        GoldCoin(0),
        Gem(0),
        Health(opts.starting_hp),
        Damage(opts.starting_damage),
        Defense(opts.starting_defense),
        PlayerEffectLoader::default(),
    )
}

/// 从局内存档恢复玩家（避免 spawn 当帧 Query 拿不到新实体）。
pub fn player_bundle_from_snapshot(_opts: &PlayerOptions, snap: &PlayerSnapshot) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        GoldCoin(snap.gold),
        Gem(snap.gems),
        Health(snap.health),
        Damage(snap.damage),
        Defense(snap.defense),
        PlayerEffectLoader::default(),
    )
}
