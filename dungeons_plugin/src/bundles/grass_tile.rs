use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::{Grass, TriggerRemaining};
use crate::effects::{
    EffectEntry, EffectPhase, EffectTrigger, GrassHealPlayer, TileEffectLoader,
};
use crate::resources::board_option::TileSize;

// ---------------------------------------------------------------------------
// 草地地块 Bundle 工厂
// ---------------------------------------------------------------------------

/// 草地地块：带 [`TileEffectLoader`] 演示「非敌人格」上的效果（踩草回复玩家生命）。
pub struct GrassTile;

impl GrassTile {
    pub fn grass_bundle(
        coord: Coordinates,
        tile_size: TileSize,
        padding: u32,
        board_size: Vec3,
    ) -> impl Bundle {
        // --- 地块效果：在「玩家触发该格」阶段为玩家加血（具体调度见 `effect_phase_dispatch_system`） ---
        let mut tile_effects = TileEffectLoader::default();
        tile_effects.push(EffectEntry::new(
            0,
            EffectTrigger::OnPhase(EffectPhase::AfterPlayerTileTrigger),
            GrassHealPlayer(1),
        ));

        (
            Name::new(format!("Tile_{}", coord)),
            Transform::from_xyz(
                (coord.x as f32 * tile_size.width as f32 + tile_size.width as f32 / 2.0)
                    - (board_size.x as f32 / 2.0),
                (coord.y as f32 * tile_size.height as f32 + tile_size.height as f32 / 2.0)
                    - (board_size.y as f32 / 2.0),
                1.0,
            ),
            Sprite {
                color: Color::srgb(0.0, 90.0, 0.0),
                custom_size: Some(Vec2::new(
                    (tile_size.width - padding) as f32,
                    (tile_size.height - padding) as f32,
                )),
                ..Default::default()
            },
            coord,
            Grass,
            TriggerRemaining::default(),
            tile_effects,
            children![],
        )
    }
}
