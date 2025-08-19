mod systems;

use bevy::prelude::*;

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
            (systems::camera::View2d::setup_camera).chain(),
        );
    }
}
