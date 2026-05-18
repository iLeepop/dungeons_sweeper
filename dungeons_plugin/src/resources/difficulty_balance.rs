//! 敌方类型抽样：随关卡扩大可抽档位上限；高关对高档位加权；数值成长由 `difficulty_factor` 驱动。

use rand::seq::IndexedRandom;
use rand::Rng;

use crate::resources::enemy_type::EnemyType;

// ---------------------------------------------------------------------------
// 档位：阶段越高，允许出现的 EnemyType 序号上限越大（Eye..EliteSlim）。
// ---------------------------------------------------------------------------

/// 当前关卡允许的最高怪物档位索引（含）：`(2 + stage).min(14)`。
pub fn max_enemy_discriminant_index(stage: u32) -> usize {
    let s = stage.max(1);
    (2 + s).min(14) as usize
}

/// 生成压力系数：`0`（偏弱势）→ `1`（偏强势），约第 15 关趋近满压。
pub fn difficulty_pressure(stage: u32) -> f32 {
    let s = stage.max(1) as f32;
    ((s - 1.0) / 14.0).clamp(0.0, 1.0)
}

/// 档位 `tier_index` 在 `0..=max_index` 内的抽样权重（随 `stage` 从弱偏置过渡到强偏置）。
pub fn enemy_tier_spawn_weight(tier_index: usize, max_index: usize, stage: u32) -> u32 {
    let cap = max_index;
    let i = tier_index.min(cap);
    let pressure = difficulty_pressure(stage);
    let weak = (cap - i + 1) as f32;
    let strong = ((i + 1) as f32).powf(1.35 + 0.65 * pressure);
    let w = weak * (1.0 - pressure) + strong * pressure;
    w.max(1.0).round() as u32
}

/// 在 `0..=max_index` 内按 [`enemy_tier_spawn_weight`] 抽样。
///
/// 单怪 HP/攻击/防御由 [`EnemyType::health`] 等与 `BoardOption::difficulty_factor` 同步上涨。
pub fn pick_weighted_enemy_type(max_index: usize, stage: u32, rng: &mut impl Rng) -> EnemyType {
    let cap = max_index.min(EnemyType::VARIANT_COUNT.saturating_sub(1));
    let choices: Vec<usize> = (0..=cap).collect();
    let picked = choices
        .choose_weighted(rng, |&i| enemy_tier_spawn_weight(i, cap, stage))
        .copied()
        .unwrap_or(0);
    EnemyType::from_discriminant_index(picked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_rises_with_stage() {
        assert!(difficulty_pressure(1) < difficulty_pressure(10));
        assert!((difficulty_pressure(20) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn high_tier_gains_weight_at_high_stage() {
        let cap = 10;
        assert!(
            enemy_tier_spawn_weight(cap, cap, 15) > enemy_tier_spawn_weight(cap, cap, 2)
        );
        assert!(
            enemy_tier_spawn_weight(0, cap, 2) > enemy_tier_spawn_weight(0, cap, 15)
        );
    }
}
