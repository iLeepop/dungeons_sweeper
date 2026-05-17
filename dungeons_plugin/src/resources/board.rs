use bevy::log;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::Enemy;
use crate::components::coordinates::Coordinates;
use crate::components::entity_status::Health;
use crate::resources::board_option::TileSize;
use crate::resources::tile_map::TileMap;
use crate::utils::bounds::Bounds2;

#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub tile_size: TileSize,
    pub bounds: Bounds2,
    pub tiles: HashMap<Coordinates, Entity>,
    pub covers: HashMap<Coordinates, Entity>,
    pub board_entity: Option<Entity>,
}

impl Board {
    /// 与 [`TileMap::safe_square_at`] 一致遍历 `coord` 的 8 邻格，累加仍为敌方实体上的 [`Health`]（不含 `coord` 本格）。
    pub fn adjacent_enemy_hp_sum_from_entities(
        &self,
        coord: Coordinates,
        enemy_health: &Query<&Health, With<Enemy>>,
    ) -> u32 {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.tiles.get(&c).copied())
            .filter_map(|entity| enemy_health.get(entity).ok())
            .map(|h| (h.0.max(0)) as u32)
            .sum()
    }

    pub fn on_board_position(
        &self,
        window: Single<&mut Window>,
        mouse_position: Vec2,
        camera_position: Vec3,
    ) -> Option<Coordinates> {
        #[cfg(feature = "debug")]
        log::info!("mouse_position: {:?}", mouse_position);
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        #[cfg(feature = "debug")]
        log::info!("camera_position: {:?}", camera_position);
        let c_pos = Vec2::new(camera_position.x, -camera_position.y);
        let pos = (mouse_position - window_size / 2.0) + c_pos;
        #[cfg(feature = "debug")]
        log::info!("bounds: {:?}", self.bounds);
        if !self.bounds.in_bounds(pos) {
            return None;
        }
        #[cfg(feature = "debug")]
        log::info!("pos: {:?}", pos);
        let tile_width = self.tile_size.width as f32;
        let tile_height = self.tile_size.height as f32;
        let x = (pos.x + self.bounds.size.x / 2.0) / tile_width;
        let y = (self.bounds.size.y - (pos.y + self.bounds.size.y / 2.0)) / tile_height;
        Some(Coordinates {
            x: x as u32,
            y: y as u32,
        })
    }

    pub fn try_uncover_tile(&mut self, _coord: Coordinates) {}

    pub fn get_tile(&self, coord: Coordinates) -> Option<&Entity> {
        self.tiles.get(&coord)
    }
}
