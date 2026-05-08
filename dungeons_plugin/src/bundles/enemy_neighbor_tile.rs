use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::{EnemyNeighbor, TriggerRemaining};
use crate::resources::board_option::TileSize;
use crate::resources::tile_map::enemy_neighbor_display_label;

pub fn enemy_neighbor_bundle(
    coord: Coordinates,
    tile_size: TileSize,
    padding: u32,
    board_size: Vec3,
    // 邻格敌方 HP 总和（用于 enemy_neighbor_display_label）。
    hp_sum_display: u32,
    counter_font: &Handle<Font>,
) -> impl Bundle {
    return (
        TriggerRemaining::zero(),
        Name::new(format!("Tile_{}", coord)),
        Transform::from_xyz(
            (coord.x as f32 * tile_size.width as f32 + tile_size.width as f32 / 2.0)
                - (board_size.x as f32 / 2.0),
            (coord.y as f32 * tile_size.height as f32 + tile_size.height as f32 / 2.0)
                - (board_size.y as f32 / 2.0),
            1.0,
        ),
        Sprite {
            color: Color::linear_rgb(0.35, 0.24, 0.12),
            custom_size: Some(Vec2::new(
                (tile_size.width - padding) as f32,
                (tile_size.height - padding) as f32,
            )),
            ..Default::default()
        },
        coord,
        EnemyNeighbor,
        children![(
            Text2d::new(enemy_neighbor_display_label(hp_sum_display)),
            TextFont {
                font: counter_font.clone(),
                font_size: 10.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            TextLayout {
                justify: Justify::Center,
                ..Default::default()
            },
            Transform::from_xyz(0., 0., 1.)
        )],
    );
}
