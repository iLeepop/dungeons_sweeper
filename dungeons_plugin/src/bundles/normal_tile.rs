use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;
use bevy::render::render_resource::Texture;

use std::collections::HashMap;

use crate::components::coordinates::Coordinates;
use crate::resources::board_option::TileSize;
use crate::resources::enemy_assets::EnemyAssets;
use crate::resources::enemy_option::EnemyOption;

pub fn normal_bundle(
    coord: Coordinates, 
    tile_size: TileSize, 
    padding: u32, 
    board_size: Vec3, 
    enemy_assets: &EnemyAssets,
    enemy_options: &EnemyOption,
    uncovers: &mut HashMap<Coordinates, Entity>,
) -> impl Bundle {
    return (
        Name::new(format!("Tile_{}", coord)),
        Transform::from_xyz((coord.x as f32 * tile_size.width as f32 + tile_size.width as f32 / 2.0) - (board_size.x as f32 / 2.0), (coord.y as f32 * tile_size.height as f32 + tile_size.height as f32 / 2.0) - (board_size.y as f32 / 2.0), 1.0),
        Sprite {
            color: Color::srgb(40.0, 0.0, 30.0),
            custom_size: Some(Vec2::new((tile_size.width - padding) as f32, (tile_size.height - padding) as f32)),
            ..Default::default()
        },
        coord,
        children![
            (
                Sprite::from_atlas_image(
                    enemy_assets.texture.clone(),
                    TextureAtlas { 
                        layout: enemy_assets.atlas_layout.clone(), 
                        index: 0 
                    }
                ),
                Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
            )
        ]
    )
}