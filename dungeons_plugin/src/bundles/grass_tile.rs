use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::grass::Grass;
use crate::resources::board_option::TileSize;

pub fn grass_bundle(
    coord: Coordinates,
    tile_size: TileSize,
    padding: u32,
    board_size: Vec3,
) -> impl Bundle {
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
            color: Color::srgb(0.0, 90.0, 0.0),
            custom_size: Some(Vec2::new(
                (tile_size.width - padding) as f32,
                (tile_size.height - padding) as f32,
            )),
            ..Default::default()
        },
        coord,
        Grass,
        children![],
    );
}
