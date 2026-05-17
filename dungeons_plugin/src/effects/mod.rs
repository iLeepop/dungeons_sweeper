//! 可扩展效果加载器：四类组件 + 优先级 + 可扩展触发枚举 + Message 驱动调度。
//!
//! 对外请优先使用本模块或 `crate::components::effects` 中的再导出（仅 `Effect` 标记仍留在 components）。

mod behavior;
mod builtin;
mod context;
mod counters;
mod dispatch;
mod entry;
mod loaders;
mod spec;
mod trigger;

pub use behavior::EffectBehavior;
pub use builtin::{GrassHealPlayer, KillBonusDamage, PLAYER_HP_MAX};
pub use context::{
    EnemyTileEffectContext, PlayerEffectContext, TileEffectContext, WorldEffectContext,
};
pub use counters::EffectCounters;
pub use dispatch::{EffectPhaseMessage, effect_phase_dispatch_system};
pub use entry::EffectEntry;
pub use loaders::{
    EnemyEffectLoader, PlayerEffectLoader, TileEffectLoader, WorldEffectHost, WorldEffectLoader,
    push_entry,
};
pub use spec::{
    ActiveEffectSpecs, SerializableEffect, build_player_loader, capture_effect_specs,
    grass_heal_amount_from_specs,
};
pub use trigger::{EffectPhase, EffectTrigger};
