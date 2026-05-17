use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::{Grass, TriggerRemaining};
use crate::effects::{EffectEntry, EffectPhase, EffectTrigger, GrassHealPlayer, TileEffectLoader};
use crate::resources::{TileSize, TilesAssets};

// ---------------------------------------------------------------------------
// 草地地块 Bundle 工厂
// ---------------------------------------------------------------------------

/// 草地地块：带 [`TileEffectLoader`] 演示「非敌人格」上的效果（踩草回复玩家生命）。
pub struct GrassTile;

impl GrassTile {
    /// `grass_heal` 大于 0 时挂载踩草回血效果（通常来自角色 `ActiveEffectSpecs`）。
    pub fn grass_bundle(
        coord: Coordinates,
        tile_size: TileSize,
        tiles_assets: &TilesAssets,
        padding: u32,
        board_size: Vec3,
        grass_heal: i8,
    ) -> impl Bundle {
        let mut tile_effects = TileEffectLoader::default();
        if grass_heal > 0 {
            tile_effects.push(EffectEntry::new(
                0,
                EffectTrigger::OnPhase(EffectPhase::AfterPlayerTileTrigger),
                GrassHealPlayer(grass_heal),
            ));
        }

        let atlas = TextureAtlas {
            layout: tiles_assets.atlas_layout.clone(),
            index: 2,
        };

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
                image: tiles_assets.texture.clone(),
                texture_atlas: Some(atlas),
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
