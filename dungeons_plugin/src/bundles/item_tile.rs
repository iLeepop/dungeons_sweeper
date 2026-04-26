use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;

use crate::components::coordinates::Coordinates;
use crate::components::item::Item;
use crate::resources::board_option::TileSize;

pub fn item_bundle(
    coord: Coordinates, 
    tile_size: TileSize, 
    padding: u32, 
    board_size: Vec3, 
) -> impl Bundle {
    return (
        Name::new(format!("Tile_{}", coord)),
        Transform::from_xyz((coord.x as f32 * tile_size.width as f32 + tile_size.width as f32 / 2.0) - (board_size.x as f32 / 2.0), (coord.y as f32 * tile_size.height as f32 + tile_size.height as f32 / 2.0) - (board_size.y as f32 / 2.0), 1.0),
        Sprite {
            color: Color::srgb(200.0, 230.0, 00.0),
            custom_size: Some(Vec2::new((tile_size.width - padding) as f32, (tile_size.height - padding) as f32)),
            ..Default::default()
        },
        coord,
        Item,
        children![
        ]
    )
}