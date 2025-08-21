use bevy::prelude::*;

use crate::resources::tile::Tile;

#[derive(Debug, Resource)]
pub struct TileMap {
    width: u32,
    height: u32,
    tiles: Vec<Vec<Tile>>,
}