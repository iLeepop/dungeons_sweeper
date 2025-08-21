use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct EntityData {
    pub health: u32,
    pub defense: u32,
    pub attack: u32
}