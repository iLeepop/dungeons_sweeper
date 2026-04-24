use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Event)]
pub struct PlayerHurt(pub u8);