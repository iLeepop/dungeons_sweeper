use bevy::log;
use bevy::prelude::*;
use rand::{Rng, rng};
use std::ops::{Deref, DerefMut};

use crate::{
    components::coordinates::Coordinates,
    resources::{
        difficulty_balance::{
            balance_enemy_loadout, max_enemy_discriminant_index, pick_weighted_enemy_type,
        },
        player_options::{DifficultyTuning, PlayerOptions},
        tile::Tile,
    },
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

/// 临近敌方格 [`Text2d`] 文案：**邻格敌方 HP 之和为 0 时显示空字符串**。
pub fn enemy_neighbor_display_label(hp_sum: u32) -> String {
    if hp_sum == 0 {
        String::new()
    } else {
        hp_sum.to_string()
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

    /// 从存档网格恢复（不调用 `set_additem`）。
    pub fn from_saved_grid(
        width: u32,
        height: u32,
        difficulty_factor: f32,
        tiles: Vec<Vec<Tile>>,
    ) -> Self {
        let mut safe_count = 0u16;
        let mut out_way_count = 0u16;
        let mut enemy_count = 0u16;
        let mut treasure_count = 0u16;
        for row in &tiles {
            for tile in row {
                match tile {
                    Tile::Safe => safe_count += 1,
                    Tile::OutWay => out_way_count += 1,
                    Tile::Enemy(_) => enemy_count += 1,
                    Tile::Treasure => treasure_count += 1,
                    _ => {}
                }
            }
        }
        TileMap {
            safe_count,
            out_way_count,
            enemy_count,
            treasure_count,
            difficulty_factor,
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

    /// 生成地图用：8 邻格内 [`Tile::Enemy`] 的 **类型默认血量** 之和（与实体生成后的初始 HP 一致）。
    pub fn enemy_health_at(&self, coordinates: Coordinates) -> i32 {
        self.safe_square_at(coordinates)
            .filter_map(|coord| {
                self.get_tile(coord).map(|tile| match tile {
                    Tile::Enemy(enemy_type) => enemy_type.health(self.difficulty_factor) as i32,
                    _ => 0,
                })
            })
            .sum()
    }

    /// 随机铺设特殊格与敌方；`stage` / `player_options` / `tuning` 用于档位抽样与总血量预算。
    pub fn set_additem(
        &mut self,
        safe_count: u16,
        out_way_count: u16,
        enemy_count: u16,
        treasure_count: u16,
        difficulty_factor: f32,
        stage: u32,
        player_options: &PlayerOptions,
        tuning: &DifficultyTuning,
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
        let max_tier = max_enemy_discriminant_index(stage);
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
                // 偏弱加权 + 阶段封顶：避免低关卡刷出满档血牛。
                self[y][x] = Tile::Enemy(pick_weighted_enemy_type(max_tier, &mut rng));
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

        // --- 总血量预算：在转为 EnemyNeighbor 之前，按需降级怪物类型 ---
        let grass_tile_count = self.count_grass_tiles();
        balance_enemy_loadout(
            self,
            difficulty_factor,
            enemy_count,
            treasure_count,
            safe_count,
            grass_tile_count,
            player_options,
            tuning,
        );

        // 设置敌方临近单位
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x: x, y: y };
                if let Tile::Grass = self[y as usize][x as usize] {
                    let health = self.enemy_health_at(coord);
                    if health > 0 {
                        let v = health.clamp(1, i32::from(u16::MAX)) as u16;
                        self[y as usize][x as usize] = Tile::EnemyNeighbor(v);
                    }
                }
            }
        }
    }

    /// 转为邻格提示之前统计仍为草地的格子数（用于估算草地总回复）。
    fn count_grass_tiles(&self) -> u32 {
        let mut n = 0u32;
        for row in self.tiles().iter() {
            for t in row.iter() {
                if matches!(t, Tile::Grass) {
                    n += 1;
                }
            }
        }
        n
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
