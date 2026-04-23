use bevy::prelude::*;
use bevy::ecs::observer::On;

use crate::events::taggle::ToggleEvent;
use crate::resources::board::Board;

pub fn taggle_consumer(
    event: On<ToggleEvent>,
    mut commands: Commands,
    mut board: ResMut<Board>,
) {
    // if let Some(tile) = board.tiles.get(&event.0) {
    //     commands.entity(*tile).despawn();
    // }
    if let Some(cover) = board.uncovers.get(&event.0) {
        commands.entity(*cover).despawn();
        board.uncovers.remove(&event.0);
    }
}