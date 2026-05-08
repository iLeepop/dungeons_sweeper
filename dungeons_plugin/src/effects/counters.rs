//! 与效果触发相关的全局计数（不参与存档）。
//!
//! 数值由对应游戏路径在发送 [`super::dispatch::EffectPhaseMessage`] **之前**更新，
//! 以便调度 System 读取时与本次 phase 一致。

use bevy::prelude::*;

// ---------------------------------------------------------------------------
// Resource：全局计数器
// ---------------------------------------------------------------------------

/// 效果系统只读/读取侧使用的全局计数；写入仅在少数 Observer 路径发生。
#[derive(Resource, Debug, Default)]
pub struct EffectCounters {
    /// 玩家成功触发「地图格子」（通过剩余次数等判定）的累计次数。
    pub player_tile_triggers: u32,
    /// 视角移动成功次数。
    pub view_moves: u32,
}
