use bevy::prelude::*;
use std::ops::{Deref, DerefMut};
use rand::{rng, Rng};

use crate::{components::coordinates::Coordinates, resources::tile::Tile};

#[derive(Resource)]
pub struct TileMap {
    monster_count: u16,
    treasure_count: u16,
    width: u32,
    height: u32,
    tiles: Vec<Vec<Tile>>,
}

impl Default for TileMap {
    fn default() -> Self {
        TileMap::new(10, 10)
    }
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> Self {
        let tiles = vec![vec![Tile::default(); height as usize]; width as usize];
        TileMap {
            monster_count: 0,
            treasure_count: 0,
            width,
            height,
            tiles,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} monsters and {} treasures:\n",
            self.width, self.height, self.monster_count, self.treasure_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }


    pub fn get_tile(&self, coord: Coordinates) -> Option<&Tile> {
        if coord.x < self.width && coord.y < self.height {
            Some(&self[coord.x as usize][coord.y as usize])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, coord: Coordinates) -> Option<&mut Tile> {
        if coord.x < self.width &&coord.x < self.height {
            Some(&mut self[coord.x as usize][coord.x as usize])
        } else {
            None
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_additem(&mut self, monster_count: u16, treasure_count: u16) {
        self.monster_count = monster_count;
        self.treasure_count = treasure_count;
        let mut remaining_monster = monster_count;
        let mut remaining_treasure = treasure_count;
        let mut one_way_out = 1;
        let mut rng = rng();
        while remaining_monster > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::Monster;
                remaining_monster -= 1;
            }
        }
        while remaining_treasure > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::Treasure;
                remaining_treasure -= 1;
            }
        }
        while one_way_out > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::OutWay;
                one_way_out -= 1;
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}