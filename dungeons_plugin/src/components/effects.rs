//! 与「效果」相关的轻量标记组件；具体加载器与调度逻辑见 [`crate::effects`]。

use bevy::prelude::*;

// ---------------------------------------------------------------------------
// 可选标记：某实体与效果系统相关（例如仅用于筛选或调试）
// ---------------------------------------------------------------------------

/// 可选的语义标记，表示该实体与效果系统有关联；不等同于挂载了某一类 [`crate::effects::PlayerEffectLoader`]。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Effect;
