use bevy::prelude::*;

use crate::components::TriggerRemaining;
use crate::resources::board_option::TileSize;
use crate::resources::tiles_assets::TilesAssets;

pub fn cover(tile_size: TileSize, padding: u32, tiles_assets: &TilesAssets) -> impl Bundle {
    let atlas = TextureAtlas {
        layout: tiles_assets.atlas_layout.clone(),
        index: 0,
    };
    return (
        TriggerRemaining::default(),
        Sprite {
            image: tiles_assets.texture.clone(),
            texture_atlas: Some(atlas),
            custom_size: Some(Vec2::new(
                (tile_size.width - padding) as f32,
                (tile_size.height - padding) as f32,
            )),
            ..Default::default()
        },
        Transform {
            scale: Vec3::new(1., 1., 0.0),
            translation: Vec3::new(0.0, 0.0, 3.0),
            ..Default::default()
        },
        // Transform::from_xyz(0.0, 0.0, 3.0),
    );
}
