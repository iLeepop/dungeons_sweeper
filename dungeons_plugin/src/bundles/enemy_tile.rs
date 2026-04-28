use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::{Coordinates, Damage, Defense, Enemy, Health};
use crate::resources::board_option::TileSize;
use crate::resources::enemy_assets::EnemyAssets;
use crate::resources::enemy_type::EnemyType;

pub fn enemy_bundle(
    coord: Coordinates,
    tile_size: TileSize,
    padding: u32,
    board_size: Vec3,
    enemy_assets: &EnemyAssets,
    enemy_type: EnemyType,
) -> impl Bundle {
    let index = match enemy_assets.enemy_atlas_layout.get(&enemy_type) {
        Some(index) => *index as usize,
        None => 0,
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
            color: Color::linear_rgb(0.95, 0.24, 0.12),
            custom_size: Some(Vec2::new(
                (tile_size.width - padding) as f32,
                (tile_size.height - padding) as f32,
            )),
            ..Default::default()
        },
        coord,
        Enemy,
        Health(enemy_type.health()),
        Damage(enemy_type.damage()),
        Defense(enemy_type.defense()),
        children![(
            Sprite::from_atlas_image(
                enemy_assets.texture.clone(),
                TextureAtlas {
                    layout: enemy_assets.atlas_layout.clone(),
                    index: index,
                }
            ),
            Transform {
                scale: Vec3::new(0.5, 0.5, 0.0),
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..Default::default()
            }
        )],
    );
}
