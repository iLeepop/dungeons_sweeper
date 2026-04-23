use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}
