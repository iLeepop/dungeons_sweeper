use bevy::prelude::*;

use std::collections::HashMap;

use crate::resources::tile::EnemyType;

#[derive(Resource)]
pub struct EnemyOption {
    pub enemy_atlas_layout: HashMap<EnemyType, u8>,
}