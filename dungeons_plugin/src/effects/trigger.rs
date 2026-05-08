//! 效果在「哪一帧 / 哪条游戏时间线」上被评估。
//!
//! ## 扩展方式
//! - 新增一种「调度时机」：给 [`EffectPhase`] 加一个变体，并在对应 Observer 或 System 里
//!   `MessageWriter::write` 一条携带该 phase 的 [`crate::effects::dispatch::EffectPhaseMessage`]。
//! - 新增一种「条目的触发条件」：给 [`EffectTrigger`] 加一个变体，并在 [`EffectTrigger::matches`] 中写清与
//!   [`crate::effects::counters::EffectCounters`]、phase 的组合关系。
//!
//! ## 若条件极度复杂
//! 可再引入 `EffectTrigger::Custom(Box<dyn Fn(&EffectCounters, EffectPhase) -> bool + Send + Sync>)` 一类分支；
//! 优点是任意布尔条件，缺点是调试难、对象安全与生命周期约束多，与「简单权重」目标相悖，故默认不启用。

use crate::effects::counters::EffectCounters;

// ---------------------------------------------------------------------------
// 调度阶段：由全局消息携带，表示「当前这条消息是在哪个时间点广播的」
// ---------------------------------------------------------------------------

/// 全局效果调度阶段（与 [`super::dispatch::EffectPhaseMessage`] 中的 phase 一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectPhase {
    /// 玩家对某一格执行翻开/触发，且该格通过了 [`crate::components::TriggerRemaining`] 等前置判定之后。
    AfterPlayerTileTrigger,
    /// 敌方攻击结算链（例如聚合伤害并触发 [`crate::events::player::PlayerHurt`]）完成之后。
    AfterEnemyAttack,
    /// 玩家视角移动成功（未撞边界）之后。
    AfterViewMove,
}

// ---------------------------------------------------------------------------
// 单条效果条目上的触发条件（与 phase 及计数资源组合）
// ---------------------------------------------------------------------------

/// 挂载在加载器里的「触发条件」；与「效果具体做什么」正交，仅决定本条是否在本次调度中执行。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectTrigger {
    /// 当且仅当本次广播的 phase 与参数相等时执行。
    OnPhase(EffectPhase),
    /// 仅在「玩家格子触发」这一 phase 下评估；且全局「格子触发次数」为 n 的正整数倍时执行（用于每 N 次行为）。
    /// 计数在 Observer 里于发送消息前递增，因此与 [`EffectCounters::player_tile_triggers`] 对齐。
    EveryNPlayerTileTriggers { n: u32 },
    /// 仅在 [`EffectPhase::AfterViewMove`] 下评估；且 [`EffectCounters::view_moves`] 为 n 的正整数倍时执行。
    EveryNViewMoves { n: u32 },
}

impl EffectTrigger {
    /// 判断在本次 `phase` 与当前计数下，条目是否应执行。
    pub fn matches(&self, phase: EffectPhase, counters: &EffectCounters) -> bool {
        match self {
            // --- 与 phase 一对一绑定 ---
            EffectTrigger::OnPhase(p) => *p == phase,
            // --- 周期性于「玩家触发格子」时刻 ---
            EffectTrigger::EveryNPlayerTileTriggers { n } => {
                phase == EffectPhase::AfterPlayerTileTrigger
                    && *n > 0
                    && counters.player_tile_triggers > 0
                    && counters.player_tile_triggers.is_multiple_of(*n)
            }
            // --- 与视角移动次数挂钩 ---
            EffectTrigger::EveryNViewMoves { n } => {
                phase == EffectPhase::AfterViewMove
                    && *n > 0
                    && counters.view_moves > 0
                    && counters.view_moves.is_multiple_of(*n)
            }
        }
    }
}
