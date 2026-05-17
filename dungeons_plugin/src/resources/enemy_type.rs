use bevy::math::ops::round;
use rand::seq::IndexedRandom;
use std::fmt::{self, Display, Formatter};

// 基础值
const BASE_ENEMY_HEALTH: [i8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const BASE_ENEMY_DAMAGE: [u8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const BASE_ENEMY_DEFENSE: [i8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
// 增长值
const HP_K: [f32; 15] = [
    1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4,
];
const DAMAGE_K: [f32; 15] = [
    1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4,
];
const DEFENSE_K: [f32; 15] = [
    1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Eye,
    MagicEye,
    Swamp,
    BlueGiant,
    RedGiant,
    Yeti,
    EliteYeti,
    Cyclops,
    Gonin,
    DoubleGonin,
    TinyMush,
    BigMush,
    MushMan,
    Slim,
    EliteSlim,
}

impl EnemyType {
    /// 枚举变体数量（与 `BASE_ENEMY_HEALTH` 等数组长度一致）。
    pub const VARIANT_COUNT: usize = 15;

    /// 与 `BASE_ENEMY_*` / `HP_K` 等常量数组下标一致。
    pub const fn index(self) -> usize {
        match self {
            EnemyType::Eye => 0,
            EnemyType::MagicEye => 1,
            EnemyType::Swamp => 2,
            EnemyType::BlueGiant => 3,
            EnemyType::RedGiant => 4,
            EnemyType::Yeti => 5,
            EnemyType::EliteYeti => 6,
            EnemyType::Cyclops => 7,
            EnemyType::Gonin => 8,
            EnemyType::DoubleGonin => 9,
            EnemyType::TinyMush => 10,
            EnemyType::BigMush => 11,
            EnemyType::MushMan => 12,
            EnemyType::Slim => 13,
            EnemyType::EliteSlim => 14,
        }
    }

    /// [`Self::index`] 的别名，便于难度预算模块读写档位。
    #[inline]
    pub const fn discriminant_index(self) -> usize {
        self.index()
    }

    /// 从下标还原类型；越界时回落到最弱档，避免生成逻辑 panic。
    pub fn from_discriminant_index(i: usize) -> Self {
        match i.clamp(0, Self::VARIANT_COUNT.saturating_sub(1)) {
            0 => EnemyType::Eye,
            1 => EnemyType::MagicEye,
            2 => EnemyType::Swamp,
            3 => EnemyType::BlueGiant,
            4 => EnemyType::RedGiant,
            5 => EnemyType::Yeti,
            6 => EnemyType::EliteYeti,
            7 => EnemyType::Cyclops,
            8 => EnemyType::Gonin,
            9 => EnemyType::DoubleGonin,
            10 => EnemyType::TinyMush,
            11 => EnemyType::BigMush,
            12 => EnemyType::MushMan,
            13 => EnemyType::Slim,
            _ => EnemyType::EliteSlim,
        }
    }

    pub fn random() -> Self {
        use EnemyType::*;
        let mut rng = rand::rng();
        [
            Eye,
            MagicEye,
            Swamp,
            BlueGiant,
            RedGiant,
            Yeti,
            EliteYeti,
            Cyclops,
            Gonin,
            DoubleGonin,
            TinyMush,
            BigMush,
            MushMan,
            Slim,
            EliteSlim,
        ]
        .choose(&mut rng)
        .unwrap()
        .clone()
    }

    pub fn health(&self, difficulty_factor: f32) -> i8 {
        let i = self.index();
        let v = BASE_ENEMY_HEALTH[i] as f32 + HP_K[i] * difficulty_factor;
        round(v) as i8
    }

    pub fn damage(&self, difficulty_factor: f32) -> u8 {
        let i = self.index();
        let v = BASE_ENEMY_DAMAGE[i] as f32 + DAMAGE_K[i] * difficulty_factor;
        round(v).clamp(0.0, u8::MAX as f32) as u8
    }

    pub fn defense(&self, difficulty_factor: f32) -> i8 {
        let i = self.index();
        let v = BASE_ENEMY_DEFENSE[i] as f32 + DEFENSE_K[i] * difficulty_factor;
        round(v) as i8
    }
}

impl Display for EnemyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EnemyType::Eye => "Eye",
                EnemyType::MagicEye => "MagicEye",
                EnemyType::Swamp => "Swamp",
                EnemyType::BlueGiant => "BlueGiant",
                EnemyType::RedGiant => "RedGiant",
                EnemyType::Yeti => "Yeti",
                EnemyType::EliteYeti => "EliteYeti",
                EnemyType::Cyclops => "Cyclops",
                EnemyType::Gonin => "Gonin",
                EnemyType::DoubleGonin => "DoubleGonin",
                EnemyType::TinyMush => "TinyMush",
                EnemyType::BigMush => "BigMush",
                EnemyType::MushMan => "MushMan",
                EnemyType::Slim => "Slim",
                EnemyType::EliteSlim => "EliteSlim",
            }
        )
    }
}
