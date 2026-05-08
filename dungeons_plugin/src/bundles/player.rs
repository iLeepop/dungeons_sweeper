use bevy::prelude::*;

use crate::components::{Damage, Defense, Health, Player, GoldCoin, Gem};
use crate::effects::PlayerEffectLoader;

pub fn player_bundle() -> impl Bundle {
    return (
        Name::new("Player"),
        Player,
        GoldCoin(0),
        Gem(0),
        Health(100),
        Damage(5),
        Defense(5),
        PlayerEffectLoader::default(),
    );
}
