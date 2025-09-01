mod components;
mod resources;
mod systems;

use bevy::{ecs::system::command, prelude::*};
use bevy::log;

#[cfg(feature = "debug")]
use crate::resources::tile;
use crate::resources::tile_map::TileMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    PreGame,
    InGame,
    OutGame,
}

pub struct DungeonsPlugin {}

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::PreGame),
            (
                        systems::camera::View2d::setup_camera, 
                        Self::create_board
                    ).chain(),
        );
    }
}

impl Default for DungeonsPlugin {
    fn default() -> Self {
        DungeonsPlugin {}
    }
}

impl DungeonsPlugin {
    pub fn new() -> Self {
        DungeonsPlugin {}
    }

    pub fn create_board(
        // commands: &mut Commands,
    ) {
        let mut tile_map = TileMap::new(10, 10);
        tile_map.set_additem(10, 10);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
