use bevy::prelude::*;

// 出生点
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Spawn;

// 草地单位
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Grass;

// 敌方单位
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Enemy;

// 敌方临近单位
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct EnemyNeighbor;

// 宝藏
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Treasure;

// 安全点
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Safe;

// 出口
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct OutWay;
