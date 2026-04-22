use std::collections::HashMap;
use bevy::prelude::*;
use bevy::log;

use crate::resources::tile_map::TileMap;
use crate::components::coordinates::Coordinates;
use crate::utils::bounds::Bounds2;
use crate::resources::board_option::TileSize;

#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub tile_size: TileSize,
    pub bounds: Bounds2,
    pub tiles: HashMap<Coordinates, Entity>,
    pub board_entity: Option<Entity>,
}

impl Board {
    pub fn on_board_position(&self, window: Single<&mut Window>, mouse_position: Vec2, camera_position: Vec3) -> Option<Coordinates> {
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
        Some(Coordinates { x: x as u32, y: y as u32 })
    }

    pub fn try_uncover_tile(&mut self, coord: Coordinates) {

    }

    pub fn get_tile(&self, coord: Coordinates) -> Option<&Entity> {
        self.tiles.get(&coord)
    }

}