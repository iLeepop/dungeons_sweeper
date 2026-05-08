//! 四类效果加载器组件 + 世界宿主标记。
//!
//! 挂载约定（与策划一致）：
//! - [`PlayerEffectLoader`]：玩家实体；
//! - [`EnemyEffectLoader`]：带 [`crate::components::Enemy`] 的地块；
//! - [`TileEffectLoader`]：草/宝藏等非敌人地块；
//! - [`WorldEffectLoader`]：与 [`WorldEffectHost`] 同实体，用于全局或跨格规则。

use bevy::prelude::*;

use crate::effects::entry::EffectEntry;

// ---------------------------------------------------------------------------
// 世界宿主标记
// ---------------------------------------------------------------------------

/// 与 [`WorldEffectLoader`] 成对出现，便于 `Query` 过滤宿主实体。
#[derive(Component, Debug, Copy, Clone, Default)]
pub struct WorldEffectHost;

// ---------------------------------------------------------------------------
// 四类加载器（结构相同，类型不同以防挂错实体）
// ---------------------------------------------------------------------------

/// 玩家身上的效果列表。
#[derive(Component, Default)]
pub struct PlayerEffectLoader {
    pub entries: Vec<EffectEntry>,
}

/// 敌人地块上的效果列表。
#[derive(Component, Default)]
pub struct EnemyEffectLoader {
    pub entries: Vec<EffectEntry>,
}

/// 非敌人地块上的效果列表。
#[derive(Component, Default)]
pub struct TileEffectLoader {
    pub entries: Vec<EffectEntry>,
}

/// 世界宿主上的效果列表。
#[derive(Component, Default)]
pub struct WorldEffectLoader {
    pub entries: Vec<EffectEntry>,
}

// ---------------------------------------------------------------------------
// 统一的 push 辅助（各加载器分别调用以保持类型路径清晰）
// ---------------------------------------------------------------------------

/// 将条目追加到 `Vec` 末尾；执行顺序由调度器按 priority 与下标排序决定。
pub fn push_entry(entries: &mut Vec<EffectEntry>, entry: EffectEntry) {
    entries.push(entry);
}

impl PlayerEffectLoader {
    pub fn push(&mut self, entry: EffectEntry) {
        push_entry(&mut self.entries, entry);
    }
}

impl EnemyEffectLoader {
    pub fn push(&mut self, entry: EffectEntry) {
        push_entry(&mut self.entries, entry);
    }
}

impl TileEffectLoader {
    pub fn push(&mut self, entry: EffectEntry) {
        push_entry(&mut self.entries, entry);
    }
}

impl WorldEffectLoader {
    pub fn push(&mut self, entry: EffectEntry) {
        push_entry(&mut self.entries, entry);
    }
}
