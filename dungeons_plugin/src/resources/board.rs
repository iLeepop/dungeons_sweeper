use bevy::prelude::*;

use crate::resources::tile_map::TileMap;

#[derive(Debug, Resource)]
pub struct Board {
    tile_map: TileMap,
}