use bevy::prelude::*;

// 生命值
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Health(pub i8);

// 防御力
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Defense(pub i8);

// 伤害值
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Damage(pub u8);
