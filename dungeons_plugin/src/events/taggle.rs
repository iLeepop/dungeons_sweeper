use bevy::prelude::*;

use crate::components::coordinates::Coordinates;

#[derive(Debug, Copy, Clone, Event)]
pub struct ToggleEvent(pub Coordinates);
