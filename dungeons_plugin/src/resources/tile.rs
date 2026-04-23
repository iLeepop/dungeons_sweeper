use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::fmt::{self, Display, Formatter};

#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Eye,
    MagicEye,
    Swamp,
    BlueGiant,
    RedGiant,
    Yeti,
    EliteYeti,
    Cyclops,
    Gonin,
    DoubleGonin,
    TinyMush,
    BigMush,
    MushMan,
    Slim,
    EliteSlim,
}

impl EnemyType {
    pub fn random() -> Self {
        use EnemyType::*;
        let mut rng = rand::rng();
        [Eye, MagicEye, Swamp, BlueGiant, RedGiant, Yeti, EliteYeti, Cyclops, Gonin, DoubleGonin, TinyMush, BigMush, MushMan, Slim, EliteSlim].choose(&mut rng).unwrap().clone()
    }
}

impl Display for EnemyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            EnemyType::Eye => "Eye",
            EnemyType::MagicEye => "MagicEye",
            EnemyType::Swamp => "Swamp",
            EnemyType::BlueGiant => "BlueGiant",
            EnemyType::RedGiant => "RedGiant",
            EnemyType::Yeti => "Yeti",
            EnemyType::EliteYeti => "EliteYeti",
            EnemyType::Cyclops => "Cyclops",
            EnemyType::Gonin => "Gonin",
            EnemyType::DoubleGonin => "DoubleGonin",
            EnemyType::TinyMush => "TinyMush",
            EnemyType::BigMush => "BigMush",
            EnemyType::MushMan => "MushMan",
            EnemyType::Slim => "Slim",
            EnemyType::EliteSlim => "EliteSlim",
        })
    }
}

#[derive(Clone, Resource)]
pub enum Tile {
    Grass,
    Enemy(EnemyType),
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
                Tile::Enemy(enemy_type) => format!("E({})", enemy_type).bright_red(),
                Tile::Treasure => "T".yellow(),
                Tile::OutWay => "#".normal(),
            }
        )
    }
}