use bevy::ecs::observer::On;
use bevy::prelude::*;

use crate::components::view::View;
use crate::effects::EffectCounters;
use crate::effects::EffectPhase;
use crate::effects::EffectPhaseMessage;
use crate::events::view_move::MoveEvent;
use crate::resources::board::Board;
use crate::resources::view2d::View2d;

pub fn view_move_consumer(
    event: On<MoveEvent>,
    board: Res<Board>,
    mut view2d: ResMut<View2d>,
    mut effect_counters: ResMut<EffectCounters>,
    mut effect_phase_writer: MessageWriter<EffectPhaseMessage>,
    cameras: Single<(&mut Transform, &mut View, &mut Camera2d), With<View>>,
) {
    let (mut transform, view, _camera) = cameras.into_inner();
    let movement = Vec3::new(
        event.0.x * view.speed as f32,
        event.0.y * view.speed as f32,
        0.0,
    );
    if !board
        .bounds
        .in_bounds(view2d.position.xy() + movement.xy() - Vec2::new(50.0, 50.0))
    {
        return;
    }
    view2d.position += movement;
    transform.translation += movement;

    // --- 效果系统：视角移动成功计数 + 广播「移动后」阶段 ---
    effect_counters.view_moves = effect_counters.view_moves.saturating_add(1);
    effect_phase_writer.write(EffectPhaseMessage {
        phase: EffectPhase::AfterViewMove,
        coord: None,
        tile: None,
    });
}
