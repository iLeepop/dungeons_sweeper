use rand::seq::IndexedRandom;
use std::fmt::{self, Display, Formatter};


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

    pub fn health(&self) -> u8 {
        match self {
            EnemyType::Eye => 1,
            EnemyType::MagicEye => 2,
            EnemyType::Swamp => 3,
            EnemyType::BlueGiant => 4,
            EnemyType::RedGiant => 5,
            EnemyType::Yeti => 6,
            EnemyType::EliteYeti => 7,
            EnemyType::Cyclops => 8,
            EnemyType::Gonin => 9,
            EnemyType::DoubleGonin => 10,
            EnemyType::TinyMush => 11,
            EnemyType::BigMush => 12,
            EnemyType::MushMan => 13,
            EnemyType::Slim => 14,
            EnemyType::EliteSlim => 15
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            EnemyType::Eye => 1,
            EnemyType::MagicEye => 2,
            EnemyType::Swamp => 3,
            EnemyType::BlueGiant => 4,
            EnemyType::RedGiant => 5,
            EnemyType::Yeti => 6,
            EnemyType::EliteYeti => 7,
            EnemyType::Cyclops => 8,
            EnemyType::Gonin => 9,
            EnemyType::DoubleGonin => 10,
            EnemyType::TinyMush => 11,
            EnemyType::BigMush => 12,
            EnemyType::MushMan => 13,
            EnemyType::Slim => 14,
            EnemyType::EliteSlim => 15
        }
    }

    pub fn defense(&self) -> u8 {
        match self {
            EnemyType::Eye => 1,
            EnemyType::MagicEye => 2,
            EnemyType::Swamp => 3,
            EnemyType::BlueGiant => 4,
            EnemyType::RedGiant => 5,
            EnemyType::Yeti => 6,
            EnemyType::EliteYeti => 7,
            EnemyType::Cyclops => 8,
            EnemyType::Gonin => 9,
            EnemyType::DoubleGonin => 10,
            EnemyType::TinyMush => 11,
            EnemyType::BigMush => 12,
            EnemyType::MushMan => 13,
            EnemyType::Slim => 14,
            EnemyType::EliteSlim => 15
        }
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