use bevy::prelude::*;

use crate::resources::tile_map::TileMap;

#[derive(Resource)]
pub struct Board {
    tile_map: TileMap,
}

impl Board {
    pub fn new(tile_map: TileMap) -> Self {
        Board { tile_map }
    }

    pub fn tile_map(&self) -> &TileMap {
        &self.tile_map
    }
}