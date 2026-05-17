use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::{Coordinates, Damage, Enemy, Health, TriggerRemaining};
use crate::resources::{EnemyAssets, EnemyType, TileSize, TilesAssets};

pub fn enemy_bundle(
    coord: Coordinates,
    tile_size: TileSize,
    tiles_assets: &TilesAssets,
    padding: u32,
    board_size: Vec3,
    enemy_assets: &EnemyAssets,
    enemy_type: EnemyType,
    difficulty_factor: f32,
) -> impl Bundle {
    let index = match enemy_assets.enemy_atlas_layout.get(&enemy_type) {
        Some(index) => *index as usize,
        None => 0,
    };
    let atlas = TextureAtlas {
        layout: tiles_assets.atlas_layout.clone(),
        index: 5,
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
        Enemy,
        TriggerRemaining::unlimit(),
        Health(enemy_type.health(difficulty_factor)),
        Damage(enemy_type.damage(difficulty_factor)),
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
