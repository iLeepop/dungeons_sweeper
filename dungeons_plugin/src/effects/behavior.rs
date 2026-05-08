//! 效果行为：仅通过四个 `apply_on_*` 钩子区分「作用在谁身上」；无额外「效果类型」枚举。
//!
//! 默认方法体为空，具体效果只实现需要的那一个钩子。

use crate::effects::context::{
    EnemyTileEffectContext, PlayerEffectContext, TileEffectContext, WorldEffectContext,
};

// ---------------------------------------------------------------------------
// Trait：单条条目的可执行逻辑
// ---------------------------------------------------------------------------

/// 一条效果的可执行逻辑；与 [`super::entry::EffectEntry`] 的 `trigger` 字段正交。
pub trait EffectBehavior: Send + Sync {
    /// 对 [`super::loaders::PlayerEffectLoader`] 中的条目调用。
    fn apply_on_player(&self, _ctx: &mut PlayerEffectContext<'_, '_, '_>) {}

    /// 对 [`super::loaders::EnemyEffectLoader`] 中的条目调用。
    fn apply_on_enemy_tile(&self, _ctx: &mut EnemyTileEffectContext<'_, '_, '_>) {}

    /// 对 [`super::loaders::TileEffectLoader`] 中的条目调用。
    fn apply_on_tile(&self, _ctx: &mut TileEffectContext<'_, '_, '_>) {}

    /// 对 [`super::loaders::WorldEffectLoader`] 中的条目调用。
    fn apply_on_world(&self, _ctx: &mut WorldEffectContext<'_, '_, '_, '_>) {}
}
