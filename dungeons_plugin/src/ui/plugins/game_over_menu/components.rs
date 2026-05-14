use bevy::prelude::*;

/// 游戏结束全屏遮罩根节点。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct GameOverMenu;

/// 重新开始：清棋盘与玩家后进入 [`crate::AppState::PreGame`]。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct GameOverRestartButton;

/// 返回主菜单：清棋盘并复位视角。
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct GameOverQuitMainMenuButton;
