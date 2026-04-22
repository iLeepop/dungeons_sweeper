use bevy::prelude::*;
use bevy::ecs::observer::On;

use crate::events::taggle::ToggleEvent;
use crate::resources::board::Board;

pub fn taggle_consumer(
    event: On<ToggleEvent>,
    mut commands: Commands,
    board: Res<Board>,
) {
    if let Some(tile) = board.tiles.get(&event.0) {
        commands.entity(*tile).despawn();
    }
}