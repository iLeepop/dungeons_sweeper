//! 玩家初始属性与「地板常数」的单一数据源，供生成关卡、效果系统与 UI 共用。

use bevy::prelude::*;

// ---------------------------------------------------------------------------
// 玩家与地板交互的数值（与策划表、难度预算公式对齐后在此调参）
// ---------------------------------------------------------------------------

/// 玩家战斗与资源条目的配置；**不**随单局内拾取而变（局内变化写在组件上）。
#[derive(Resource, Clone, Debug)]
pub struct PlayerOptions {
    /// 生命上限；草地/安全点回复、HUD 血条分母均引用此值。
    pub max_hp: i8,
    /// 开局当前生命（通常 ≤ max_hp）。
    pub starting_hp: i8,
    /// 开局攻击力；宝藏格会在此基础上一段时间内累加 `treasure_damage_bonus`。
    pub starting_damage: u8,
    /// 开局护盾（先吸收伤害，耗尽后再扣血）。
    pub starting_defense: i8,
    /// 每触发一次草地格回复的生命。
    pub grass_heal_per_trigger: i8,
    /// 每触发一次安全格回复的生命（开发阶段默认 10）。
    pub safe_heal_per_trigger: i8,
    /// 每触发一次宝藏格增加的攻击力（开发阶段默认 +1）。
    pub treasure_damage_bonus: u8,
}

impl Default for PlayerOptions {
    fn default() -> Self {
        Self {
            max_hp: 100,
            starting_hp: 10,
            starting_damage: 3,
            starting_defense: 0,
            grass_heal_per_trigger: 1,
            safe_heal_per_trigger: 10,
            treasure_damage_bonus: 1,
        }
    }
}

// ---------------------------------------------------------------------------
// 敌方生成「档位 + 总血量预算」的可调参数（与 PlayerOptions 分离以免臃肿）
// ---------------------------------------------------------------------------

/// 难度预算公式系数：`effective_atk`、`heal_budget`、`hp_cap_*` 等均依赖此处。
#[derive(Resource, Clone, Debug)]
pub struct DifficultyTuning {
    /// φ：假设玩家在打完所有怪之前只会捡到一部分宝藏（0~1）。
    pub treasure_atk_fraction: f32,
    /// η：把「敌方总血量」与「玩家续航池」拉到同一量级时的缩放。
    pub survive_eta: f32,
    /// 期望每只怪需要的基础攻击次数乘子（用于击杀方向的总 HP 上限）。
    pub avg_hits_per_monster: f32,
}

impl Default for DifficultyTuning {
    fn default() -> Self {
        Self {
            treasure_atk_fraction: 0.5,
            survive_eta: 1.0,
            avg_hits_per_monster: 2.5,
        }
    }
}
