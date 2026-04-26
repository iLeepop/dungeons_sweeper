use bevy::prelude::*;

use crate::resources::enemy_type::EnemyType;

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Clone, Resource)]
pub enum Tile {
    Grass,
    Enemy(EnemyType),
    EnemyNeighbor(u8),
    Treasure,
    OutWay,
    Safe,
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