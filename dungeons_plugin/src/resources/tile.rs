use bevy::prelude::*;

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Clone, Resource)]
pub enum Tile {
    Grass,
    Monster,
    Treasure,
    OutWay,
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
                Tile::Monster => "M".bright_red(),
                Tile::Treasure => "T".yellow(),
                Tile::OutWay => "#".normal(),
            }
        )
    }
}