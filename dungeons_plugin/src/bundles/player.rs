use bevy::prelude::*;

use crate::components::{Damage, Defense, Health, Player};

pub fn player_bundle() -> impl Bundle {
    return (
        Name::new("Player"),
        Player,
        Health(100),
        Damage(5),
        Defense(5),
    );
}
