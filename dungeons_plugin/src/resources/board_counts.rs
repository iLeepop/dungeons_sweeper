//! 地板显式类型数量：按地图面积与关卡阶段计算，并在超容时裁剪。

use super::board_option::BoardOption;

/// 与 [`crate::resources::board_option::BoardOption`] 中四类计数对应（不含出生点）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileKindCounts {
    pub safe: u16,
    pub out_way: u16,
    pub monster: u16,
    pub treasure: u16,
}

/// 出生点固定占用 1 格，与 `TileMap::set_additem` 校验一致。
pub const SPAWN_RESERVED: u32 = 1;

/// 关卡对应的正方形棋盘边长（宽=高），**阶梯式增长**：前期关卡密、后期放缓并封顶。
///
/// - 与 [`apply_stage_to_board_option`] 同步使用，保证 `TileMap` 面积随 `StageConfig.stage` 扩大。
/// - 计数类公式仍用 [`counts_for_stage`]，其 `area = side * side` 会随本边长自动缩放。
pub fn map_side_for_stage(stage: u32) -> u32 {
    let s = stage.max(1);
    match s {
        1 => 8,
        2..=3 => 10,
        4..=5 => 13,
        6..=7 => 14,
        8..=10 => 17,
        11..=13 => 18,
        // 之后每 2 关 +1 边长，上限 1000，避免单局过大。
        _ => (18 + (s.saturating_sub(19)) + (s.saturating_sub(19)) * 2).min(1000),
    }
}

/// 关卡对应的地图尺寸（当前策略为正方形）。
#[inline]
pub fn map_size_for_stage(stage: u32) -> (u32, u32) {
    let n = map_side_for_stage(stage);
    (n, n)
}

pub fn difficulty_factor_for_stage(stage: u32) -> f32 {
    let s = stage.max(1) as f32;
    1.0 + 0.18 * (s - 1.0).max(0.0)
}

/// 按规划公式得到理想计数（未做超容裁剪）。
pub fn desired_counts(area: u32, stage: u32) -> TileKindCounts {
    let s = stage.max(1);
    let sf = (s - 1) as f32;
    let a = area as f32;

    let safe = (1 + (s.saturating_sub(1)) / 3).min(3) as u16;

    let out_cap = ((0.05 * a).floor() as u32).max(1);
    let out_way = if s >= 8 {
        out_cap.min(2).max(1) as u16
    } else {
        1
    };

    let monster_raw = (a * (0.08 + 0.006 * sf)).round() as i32;
    let m_low = (3.0_f32).max((0.05 * a).ceil()) as i32;
    let m_high = (0.18 * a).floor() as i32;
    let monster = monster_raw.clamp(m_low, m_high.max(m_low)) as u16;

    let treasure_raw = (a * (0.05 + 0.004 * sf)).round() as i32;
    let t_low = (1.0_f32).max((0.03 * a).floor()) as i32;
    let t_high = (0.12 * a).floor() as i32;
    let treasure = treasure_raw.clamp(t_low, t_high.max(t_low)) as u16;

    TileKindCounts {
        safe,
        out_way,
        monster,
        treasure,
    }
}

pub fn sum_with_spawn_reserved(area_components: &TileKindCounts) -> u32 {
    SPAWN_RESERVED
        + area_components.safe as u32
        + area_components.out_way as u32
        + area_components.monster as u32
        + area_components.treasure as u32
}

fn trim_floors(area: u32) -> (u16, u16) {
    let a = area as f32;
    let treasure_floor = (1.0_f32).max((0.03 * a).floor()) as u16;
    let monster_floor = (3.0_f32).max((0.05 * a).ceil()) as u16;
    (treasure_floor, monster_floor)
}

/// 超容时按 treasure → monster → safe 削减，**不改动出口**；必要时打破下限直至装入。
pub fn trim_counts_to_capacity(area: u32, c: &mut TileKindCounts) {
    let (treasure_floor, monster_floor) = trim_floors(area);

    while sum_with_spawn_reserved(c) > area {
        if c.treasure > treasure_floor {
            c.treasure -= 1;
            continue;
        }
        if c.monster > monster_floor {
            c.monster -= 1;
            continue;
        }
        if c.safe > 0 {
            c.safe -= 1;
            continue;
        }
        if c.treasure > 0 {
            c.treasure -= 1;
            continue;
        }
        if c.monster > 0 {
            c.monster -= 1;
            continue;
        }
        break;
    }
}

/// 计算并裁剪后的计数，保证 `1 + safe + out + monster + treasure <= area`（在可达时）。
pub fn counts_for_stage(area: u32, stage: u32) -> TileKindCounts {
    let mut c = desired_counts(area, stage);
    trim_counts_to_capacity(area, &mut c);
    c
}

/// 按阶段写入 `BoardOption`：**地图边长（阶梯）**、四类数量与 `difficulty_factor`。
pub fn apply_stage_to_board_option(board: &mut BoardOption, stage: u32) {
    board.map_size = map_size_for_stage(stage);
    let (w, h) = board.map_size;
    let area = w.saturating_mul(h);
    let c = counts_for_stage(area, stage);
    board.safe_count = c.safe;
    board.out_way_count = c.out_way;
    board.monster_count = c.monster;
    board.treasure_count = c.treasure;
    board.difficulty_factor = difficulty_factor_for_stage(stage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_grows_with_stage() {
        assert!(map_side_for_stage(1) < map_side_for_stage(8));
        assert!(map_side_for_stage(8) <= map_side_for_stage(30));
        assert_eq!(map_side_for_stage(1), 5);
        assert_eq!(map_side_for_stage(20), 12);
        assert_eq!(map_side_for_stage(100), 15);
    }

    #[test]
    fn capacity_holds_for_small_board() {
        let area = 25;
        let c = counts_for_stage(area, 1);
        assert!(sum_with_spawn_reserved(&c) <= area);
    }
}
