use bevy::prelude::*;

use std::collections::HashMap;

use crate::resources::enemy_type::EnemyType;

#[derive(Resource)]
pub struct EnemyAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub enemy_atlas_layout: HashMap<EnemyType, u8>,
}
