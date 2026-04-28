use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Event)]
pub struct MoveEvent(pub Vec2);
