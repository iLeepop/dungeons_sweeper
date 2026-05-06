use bevy::prelude::*;

use crate::resources::enemy_type::EnemyType;

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Clone, Resource)]
pub enum Tile {
    Spawn, // 出生点
    Grass, // 草地单位
    Enemy(EnemyType), // 敌方单位
    EnemyNeighbor(u8), // 敌方临近单位
    Treasure, // 宝藏
    OutWay, // 出口
    Safe, // 安全点
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Grass
    }
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Spawn => "S".normal(),
                Tile::Grass => "G".normal(),
                Tile::Enemy(enemy_type) => format!("E({})", enemy_type).bright_red(),
                Tile::EnemyNeighbor(count) => format!("N({})", count).bright_blue(),
                Tile::Treasure => "T".yellow(),
                Tile::OutWay => "#".normal(),
                Tile::Safe => "S".green(),
            }
        )
    }
}

// 敌方临近单位信息
pub struct EnemyNeighborInfo {
    pub enemy_hp_sum: i8,
    pub enemy_count: u8,
    pub treasure_count: u8,
}
