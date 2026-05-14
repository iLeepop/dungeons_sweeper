use bevy::prelude::*;

/// 关卡完成后的全屏菜单根节点。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct NextLevelMenu;

/// 升关并重建棋盘后继续游戏。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct NextLevelContinueButton;

/// 放弃进入下一关并返回主菜单。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct NextLevelQuitMainMenuButton;
