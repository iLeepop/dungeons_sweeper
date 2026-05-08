//! 少量内置效果示例，便于在 Bundle 中演示加载器用法。

use crate::effects::behavior::EffectBehavior;
use crate::effects::context::TileEffectContext;

// ---------------------------------------------------------------------------
// 草地：触发后为玩家回复生命（上限与 HUD 逻辑中使用的 100 对齐）
// ---------------------------------------------------------------------------

/// 玩家生命上限（与 [`crate::observers::player_action`] 中 HPBar 的 max 注释一致）。
pub const PLAYER_HP_MAX: i8 = 100;

/// 踩在草地上时，为玩家增加固定生命值。
#[derive(Debug, Clone, Copy)]
pub struct GrassHealPlayer(pub i8);

impl EffectBehavior for GrassHealPlayer {
    fn apply_on_tile(&self, ctx: &mut TileEffectContext<'_, '_, '_>) {
        match &mut ctx.player_health {
            Some(h) => {
                h.0 = (h.0 + self.0).min(PLAYER_HP_MAX);
            }
            None => {}
        }
    }
}
