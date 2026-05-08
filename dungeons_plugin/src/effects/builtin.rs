//! 少量内置效果示例，便于在 Bundle 中演示加载器用法。

use crate::effects::behavior::EffectBehavior;
use crate::effects::context::TileEffectContext;

// ---------------------------------------------------------------------------
// 草地：触发后为玩家回复生命（上限由调度器写入 [`TileEffectContext::player_hp_cap`]）
// ---------------------------------------------------------------------------

/// 兼容旧代码的默认生命上限（与 [`crate::resources::PlayerOptions::default`] 一致）。
pub const PLAYER_HP_MAX: i8 = 100;

/// 踩在草地上时，为玩家增加固定生命值（每格数值由 [`crate::bundles::GrassTile`] 绑定）。
#[derive(Debug, Clone, Copy)]
pub struct GrassHealPlayer(pub i8);

impl EffectBehavior for GrassHealPlayer {
    fn apply_on_tile(&self, ctx: &mut TileEffectContext<'_, '_, '_>) {
        match &mut ctx.player_health {
            Some(h) => {
                // 使用选项中的上限，避免 HUD / 草地回复 / 难度公式三者不一致。
                let cap = ctx.player_hp_cap as i32;
                h.0 = (h.0 as i32 + self.0 as i32)
                    .clamp(i8::MIN as i32, cap.min(i8::MAX as i32)) as i8;
            }
            None => {}
        }
    }
}
