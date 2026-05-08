//! 出口格触发后推迟到帧末再重建棋盘，避免在 Observer 中持有 `ResMut<Board>` 时 `remove_resource`。

use bevy::prelude::*;

/// 若本帧需要重建棋盘（例如踩到出口），由 Observer 置 `true`，[`crate::flush_pending_board_rebuild`] 消费后复位。
#[derive(Resource, Default)]
pub struct PendingBoardRebuild(pub bool);

impl PendingBoardRebuild {
    /// 取出并重置「待重建」标记；返回 `true` 表示本帧应执行重建。
    pub fn take_pending(&mut self) -> bool {
        core::mem::replace(&mut self.0, false)
    }
}
