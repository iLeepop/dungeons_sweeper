use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::{Safe, TriggerRemaining};
use crate::resources::{TileSize, TilesAssets};

pub fn safe_bundle(
    coord: Coordinates,
    tile_size: TileSize,
    tiles_assets: &TilesAssets,
    padding: u32,
    board_size: Vec3,
) -> impl Bundle {
    let atlas = TextureAtlas {
        layout: tiles_assets.atlas_layout.clone(),
        index: 6,
    };
    return (
        Name::new(format!("Tile_{}", coord)),
        Transform::from_xyz(
            (coord.x as f32 * tile_size.width as f32 + tile_size.width as f32 / 2.0)
                - (board_size.x as f32 / 2.0),
            (coord.y as f32 * tile_size.height as f32 + tile_size.height as f32 / 2.0)
                - (board_size.y as f32 / 2.0),
            1.0,
        ),
        Sprite {
            image: tiles_assets.texture.clone(),
            texture_atlas: Some(atlas),
            custom_size: Some(Vec2::new(
                (tile_size.width - padding) as f32,
                (tile_size.height - padding) as f32,
            )),
            ..Default::default()
        },
        coord,
        Safe,
        TriggerRemaining::new(1),
        children![],
    );
}
