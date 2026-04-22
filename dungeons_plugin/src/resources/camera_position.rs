use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraPosition {
    pub position: Vec2,
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition { position: Vec2::new(0.0, 0.0) }
    }
}