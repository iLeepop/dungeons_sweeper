use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CharacterId {
    Herbalist = 0,
    Slayer = 1,
}

impl CharacterId {
    pub const COUNT: usize = 2;

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(i: u8) -> Option<Self> {
        match i {
            0 => Some(CharacterId::Herbalist),
            1 => Some(CharacterId::Slayer),
            _ => None,
        }
    }

    pub fn to_index(self) -> u8 {
        self as u8
    }

    /// 向右（下一角色）。
    pub fn next(self) -> Self {
        let i = (self.index() + 1) % Self::COUNT;
        CharacterId::from_index(i as u8).unwrap_or(self)
    }

    /// 向左（上一角色）。
    pub fn prev(self) -> Self {
        let i = (self.index() + Self::COUNT - 1) % Self::COUNT;
        CharacterId::from_index(i as u8).unwrap_or(self)
    }
}

impl Default for CharacterId {
    fn default() -> Self {
        CharacterId::Herbalist
    }
}
