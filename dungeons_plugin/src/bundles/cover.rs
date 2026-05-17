use bevy::prelude::*;

use crate::components::TriggerRemaining;
use crate::resources::board_option::TileSize;
use crate::resources::tiles_assets::TilesAssets;

pub fn cover(tile_size: TileSize, padding: u32, tiles_assets: &TilesAssets) -> impl Bundle {
    return (
        TriggerRemaining::default(),
        Sprite::from_atlas_image(
            tiles_assets.texture.clone(),
            TextureAtlas {
                layout: tiles_assets.atlas_layout.clone(),
                index: 0,
            }
        ),
        Transform {
            scale: Vec3::new(0.5, 0.5, 0.0),
            translation: Vec3::new(0.0, 0.0, 2.0),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 3.0),
    );
}
