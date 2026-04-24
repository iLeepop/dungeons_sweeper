use bevy::window::Window;
use bevy::prelude::*;
use bevy::log;


use crate::events::taggle::ToggleEvent;
use crate::events::view_move::MoveEvent;
use crate::resources::board::Board;
use crate::resources::view2d::View2d;

pub fn input_handler(
    mut commands: Commands,
    window: Single<&mut Window>,
    board: Res<Board>,
    view2d: Res<View2d>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        let position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };
        let camera_position = view2d.position;
        let coordinates = board.on_board_position(window, position, camera_position);
        if let Some(coordinates) = coordinates {
            #[cfg(feature = "debug")]
            log::info!("coordinates: {:?}", coordinates);
            commands.trigger(ToggleEvent(coordinates));
        }
    } else if input.just_pressed(MouseButton::Right) {
        let position = match window.cursor_position() {
            Some(position) => position,
            None => return,
        };
        let camera_position = view2d.position;
        let coordinates = board.on_board_position(window, position, camera_position);
        if let Some(coordinates) = coordinates {
            let tile = board.tile_map.get_tile(coordinates);
        }
    } else if input.pressed(MouseButton::Middle) {

    }
}

pub fn keyboard_input_handler(
    mut commands: Commands,
    window: Single<&mut Window>,
    board: Res<Board>,
    view2d: Res<View2d>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyW) {
        commands.trigger(MoveEvent(Vec2::new(0.0, 1.0)));
    } 
    if input.pressed(KeyCode::KeyS) {
        commands.trigger(MoveEvent(Vec2::new(0.0, -1.0)));
    } 
    if input.pressed(KeyCode::KeyA) {
        commands.trigger(MoveEvent(Vec2::new(-1.0, 0.0)));
    } 
    if input.pressed(KeyCode::KeyD) {
        commands.trigger(MoveEvent(Vec2::new(1.0, 0.0)));
    }
}