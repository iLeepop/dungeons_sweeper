//! 敌方类型抽样与总血量预算：避免「少量随机高阶怪」导致无解局面，贴合扫雷式信息博弈。

use rand::seq::IndexedRandom;
use rand::{Rng, rng};

use crate::resources::enemy_type::EnemyType;
use crate::resources::player_options::{DifficultyTuning, PlayerOptions};
use crate::resources::tile::Tile;
use crate::resources::tile_map::TileMap;

// ---------------------------------------------------------------------------
// 与 safe_bundle 中 `TriggerRemaining::new(3)` 对齐，用于估算安全格总回复量。
// ---------------------------------------------------------------------------

/// 安全格默认可触发次数上限（与 [`crate::bundles::safe_bundle`] 保持一致）。
pub const SAFE_TILE_TRIGGER_CAP: i32 = 3;

// ---------------------------------------------------------------------------
// 档位：阶段越高，允许出现的 EnemyType 序号上限越大（偏弱权重更高）。
// ---------------------------------------------------------------------------

/// 当前关卡允许的最高怪物档位索引（含）：`(2 + stage).min(14)`，对应 Eye..EliteSlim。
pub fn max_enemy_discriminant_index(stage: u32) -> usize {
    let s = stage.max(1);
    (2 + s).min(14) as usize
}

/// 在 `0..=max_index` 内按偏弱加权抽样（索引越小权重越大）。
pub fn pick_weighted_enemy_type(max_index: usize, rng: &mut impl Rng) -> EnemyType {
    let cap = max_index.min(EnemyType::VARIANT_COUNT.saturating_sub(1));
    let choices: Vec<usize> = (0..=cap).collect();
    let picked = choices
        .choose_weighted(rng, |&i| (cap - i + 1) as u32)
        .copied()
        .unwrap_or(0);
    EnemyType::from_discriminant_index(picked)
}

// ---------------------------------------------------------------------------
// 总血量预算：击杀预算与生存预算取较小，再通过降级怪物类型压 `sum_hp`。
// ---------------------------------------------------------------------------

/// 在铺设邻格展示之前调用：按预算把部分怪物降为更弱类型。
///
/// - **击杀预算**：估算玩家在捡到部分宝藏后的有效攻击力 × 期望击打次数。
/// - **生存预算**：用「生命 + 护盾 + 草地/安全格近似总回复」粗略对齐敌方「血量总值」尺度。
pub fn balance_enemy_loadout(
    tile_map: &mut TileMap,
    difficulty_factor: f32,
    monster_count: u16,
    treasure_count: u16,
    safe_count: u16,
    grass_tile_count: u32,
    player_options: &PlayerOptions,
    tuning: &DifficultyTuning,
) {
    let heal_budget = grass_tile_count as f32 * player_options.grass_heal_per_trigger as f32
        + safe_count as f32
            * SAFE_TILE_TRIGGER_CAP as f32
            * player_options.safe_heal_per_trigger as f32;

    let effective_atk = player_options.starting_damage as f32
        + tuning.treasure_atk_fraction
            * treasure_count as f32
            * player_options.treasure_damage_bonus as f32;

    let hp_cap_kill = effective_atk * monster_count as f32 * tuning.avg_hits_per_monster.max(0.1);

    let sustain_pool =
        player_options.starting_hp as f32 + player_options.starting_defense as f32 + heal_budget;
    let hp_cap_survive = tuning.survive_eta.max(0.1) * sustain_pool;

    let target_cap = hp_cap_kill.min(hp_cap_survive).floor().max(1.0) as i32;

    let mut sum_hp = sum_enemy_hp_on_map(tile_map, difficulty_factor);
    let max_iters = (monster_count as usize).saturating_mul(32).max(64);
    let mut rng = rng();

    for _ in 0..max_iters {
        if sum_hp <= target_cap {
            break;
        }
        let Some((x, y)) = random_enemy_cell(tile_map, &mut rng) else {
            break;
        };
        let Tile::Enemy(ty) = tile_map[y][x] else {
            continue;
        };
        let idx = ty.discriminant_index();
        if idx == 0 {
            continue;
        }
        let weaker = EnemyType::from_discriminant_index(idx - 1);
        sum_hp -= ty.health(difficulty_factor) as i32;
        sum_hp += weaker.health(difficulty_factor) as i32;
        tile_map[y][x] = Tile::Enemy(weaker);
    }
}

fn sum_enemy_hp_on_map(tile_map: &TileMap, difficulty_factor: f32) -> i32 {
    let mut s = 0i32;
    for row in tile_map.tiles().iter() {
        for t in row.iter() {
            if let Tile::Enemy(ty) = t {
                s += ty.health(difficulty_factor) as i32;
            }
        }
    }
    s
}

fn random_enemy_cell(tile_map: &TileMap, rng: &mut impl Rng) -> Option<(usize, usize)> {
    let h = tile_map.height() as usize;
    let w = tile_map.width() as usize;
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if matches!(tile_map[y][x], Tile::Enemy(_)) {
                coords.push((x, y));
            }
        }
    }
    coords.choose(rng).copied()
}
