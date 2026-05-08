use bevy::prelude::*;

use crate::components::{Damage, Defense, Health, Player, GoldCoin, Gem};
use crate::effects::PlayerEffectLoader;
use crate::resources::PlayerOptions;

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
