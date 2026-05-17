use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct TilesAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}
