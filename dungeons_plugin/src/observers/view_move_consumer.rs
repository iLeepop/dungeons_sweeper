use bevy::prelude::*;
use bevy::ecs::observer::On;

use crate::components::view::View;
use crate::events::view_move::MoveEvent;
use crate::resources::view2d::View2d;
use crate::resources::board::Board;

pub fn view_move_consumer(
    event: On<MoveEvent>,
    board: Res<Board>,
    mut view2d: ResMut<View2d>,
    mut cameras: Query<(&mut Transform, &mut View), With<View>>,
) {
    for (mut transform, view) in cameras.iter_mut() {
        let movement = Vec3::new(event.0.x * view.speed as f32, event.0.y * view.speed as f32, 0.0);
        if !board.bounds.in_bounds(view2d.position.xy() + movement.xy() - Vec2::new(50.0, 50.0)) {
            continue;
        }
        view2d.position += movement;
        transform.translation += movement;
    }
}