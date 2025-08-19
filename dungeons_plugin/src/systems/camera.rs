use bevy::prelude::*;

#[derive()]
pub struct View2d {
    camera: Camera2d
}

impl View2d {
    pub fn setup_camera(
        mut commands: Commands
    ) {
        commands.spawn(Camera2d);
    }
}
