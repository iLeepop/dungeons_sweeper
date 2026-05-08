//! 四类加载器各自的应用上下文：效果实现只应读写「语义上允许」的实体与组件。
//!
//! 调度器负责从 `Query` 中取出常用组件句柄填入上下文；更冷门的组件可在实现里通过
//! `commands` / 后续扩展的 `World` 访问（当前未暴露 `&mut World` 以保持调度器简单）。

use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::components::entity_status::Health;
use crate::resources::board::Board;

// ---------------------------------------------------------------------------
// 玩家加载器上下文
// ---------------------------------------------------------------------------

/// 挂在 [`super::loaders::PlayerEffectLoader`] 上的效果在 `apply_on_player` 时收到的环境。
pub struct PlayerEffectContext<'w, 's, 'a> {
    /// 延迟命令队列（Bevy 0.18 中 `Commands` 为双生命周期参数）。
    pub commands: &'a mut Commands<'w, 's>,
    /// 玩家实体。
    pub player: Entity,
    /// 本次触发对应的地图坐标（若有）。
    pub trigger_coord: Option<Coordinates>,
    /// 本次触发对应的地块实体（若有）。
    pub trigger_tile: Option<Entity>,
    /// 玩家生命值（单玩家场景下由调度器解析）。
    pub player_health: Option<Mut<'a, Health>>,
}

// ---------------------------------------------------------------------------
// 敌人地块加载器上下文
// ---------------------------------------------------------------------------

/// 挂在带 [`crate::components::Enemy`] 的地块上的效果所使用的环境。
pub struct EnemyTileEffectContext<'w, 's, 'a> {
    pub commands: &'a mut Commands<'w, 's>,
    /// 带 Enemy 的地块实体。
    pub tile: Entity,
    pub coord: Coordinates,
    pub player: Entity,
    pub tile_health: Option<Mut<'a, Health>>,
    pub player_health: Option<Mut<'a, Health>>,
}

// ---------------------------------------------------------------------------
// 非敌人地块加载器上下文（草、宝藏等）
// ---------------------------------------------------------------------------

/// 挂在无敌人标记的地块上的 [`super::loaders::TileEffectLoader`] 所使用的环境。
pub struct TileEffectContext<'w, 's, 'a> {
    pub commands: &'a mut Commands<'w, 's>,
    pub tile: Entity,
    pub coord: Coordinates,
    pub player: Entity,
    pub tile_health: Option<Mut<'a, Health>>,
    pub player_health: Option<Mut<'a, Health>>,
}

// ---------------------------------------------------------------------------
// 世界加载器上下文
// ---------------------------------------------------------------------------

/// 挂在 [`super::loaders::WorldEffectHost`] 上的 [`super::loaders::WorldEffectLoader`] 所使用的环境。
pub struct WorldEffectContext<'w, 's, 'a, 'b> {
    pub commands: &'a mut Commands<'w, 's>,
    /// 世界效果宿主实体（用于区分多个 host 时的日志或子实体操作）。
    pub host: Entity,
    pub player: Entity,
    pub trigger_coord: Option<Coordinates>,
    pub trigger_tile: Option<Entity>,
    pub board: &'b Board,
    pub player_health: Option<Mut<'a, Health>>,
}
