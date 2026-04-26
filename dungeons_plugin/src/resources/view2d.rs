use bevy::prelude::*;

#[derive(Resource)]
pub struct View2d {
    pub camera: Entity,
    pub position: Vec3,
}
