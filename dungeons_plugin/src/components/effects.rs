use bevy::prelude::*;

// 效果
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct Effect;

// 激活中的效果
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct ActivateEffect;

// 效果实例
pub trait EffectInstance {

}