use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct TileSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct BoardOption {
    pub map_size: (u32, u32),
    pub tile_size: TileSize,
    pub padding: u32,
    pub counter_font: Handle<Font>,
    pub safe_count: u16,
    pub out_way_count: u16,
    pub monster_count: u16,
    pub treasure_count: u16,
}

impl Default for TileSize {
    fn default() -> Self {
        TileSize { width: 10, height: 10 }
    }
}

impl Default for BoardOption {
    fn default() -> Self {
        BoardOption { 
            map_size: (10, 10), 
            tile_size: TileSize::default(), 
            padding: 0,
            counter_font: Handle::default(),
            safe_count: 1,
            out_way_count: 1,
            monster_count: 10, 
            treasure_count: 10,
        }
    }
}

impl BoardOption {
    pub fn map_size(&self) -> (u32, u32) {
        self.map_size
    }

    pub fn tile_size(&self) -> TileSize {
        self.tile_size
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn safe_count(&self) -> u16 {
        self.safe_count
    }

    pub fn monster_count(&self) -> u16 {
        self.monster_count
    }

    pub fn treasure_count(&self) -> u16 {
        self.treasure_count
    }
}