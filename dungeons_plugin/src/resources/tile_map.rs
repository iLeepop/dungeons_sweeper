use bevy::log;
use bevy::prelude::*;
use rand::{Rng, rng};
use std::ops::{Deref, DerefMut};

use crate::{
    components::coordinates::Coordinates,
    resources::{enemy_type::EnemyType, tile::Tile},
};

const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Resource)]
pub struct TileMap {
    safe_count: u16,
    out_way_count: u16,
    enemy_count: u16,
    treasure_count: u16,
    difficulty_factor: f32,
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
            safe_count: 0,
            out_way_count: 0,
            enemy_count: 0,
            treasure_count: 0,
            difficulty_factor: 1.0,
            width,
            height,
            tiles,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} enemies and {} treasures:\n",
            self.width, self.height, self.enemy_count, self.treasure_count
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
            Some(&self[coord.y as usize][coord.x as usize])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, coord: Coordinates) -> Option<&mut Tile> {
        if coord.x < self.width && coord.y < self.height {
            Some(&mut self[coord.y as usize][coord.x as usize])
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

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn enemy_health_at(&self, coordinates: Coordinates) -> i8 {
        self.safe_square_at(coordinates)
            .filter_map(|coord| {
                self.get_tile(coord).map(|tile| match tile {
                    Tile::Enemy(enemy_type) => enemy_type.health(self.difficulty_factor),
                    _ => 0,
                })
            })
            .sum()
    }

    pub fn set_additem(
        &mut self,
        safe_count: u16,
        out_way_count: u16,
        enemy_count: u16,
        treasure_count: u16,
        difficulty_factor: f32,
    ) {
        self.difficulty_factor = difficulty_factor;
        if (1 + safe_count + out_way_count + enemy_count + treasure_count) as u32
            > self.width * self.height
        {
            #[cfg(feature = "debug")]
            log::error!(
                "safe_count + out_way_count + monster_count + treasure_count > width * height"
            );
            return;
        }
        self.enemy_count = enemy_count;
        self.treasure_count = treasure_count;
        self.safe_count = safe_count;
        self.out_way_count = out_way_count;
        let mut remaining_enemy = enemy_count;
        let mut remaining_treasure = treasure_count;
        let mut one_way_out = out_way_count;
        let mut safe_count = safe_count;
        let mut rng = rng();
        // 出生点（不占用 monster_count）
        {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::Spawn;
            }
        }
        // 出口与安全点优先于怪物/宝藏，降低被特殊格挤占的概率
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
        while safe_count > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::Safe;
                safe_count -= 1;
            }
        }
        // 敌方单位
        while remaining_enemy > 0 {
            let (x, y) = (
                rng.random_range(0..self.width) as usize,
                rng.random_range(0..self.height) as usize,
            );
            if let Tile::Grass = self[y][x] {
                self[y][x] = Tile::Enemy(EnemyType::random());
                remaining_enemy -= 1;
            }
        }
        // 宝藏
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
        // 设置敌方临近单位
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x: x, y: y };
                if let Tile::Grass = self[y as usize][x as usize] {
                    let health = self.enemy_health_at(coord);
                    if health > 0 {
                        self[y as usize][x as usize] = Tile::EnemyNeighbor(health as u8);
                    }
                }
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
