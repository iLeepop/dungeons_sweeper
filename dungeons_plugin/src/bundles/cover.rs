use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use crate::resources::board_option::TileSize;

pub fn cover(tile_size: TileSize, padding: u32) -> impl Bundle {
    return (
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new((tile_size.width - padding) as f32, (tile_size.height - padding) as f32)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
    )
}