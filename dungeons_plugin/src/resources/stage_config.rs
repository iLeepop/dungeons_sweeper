use bevy::prelude::*;

/// 当前关卡阶段（从 1 起），驱动地板数量与 `difficulty_factor`。
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageConfig {
    pub stage: u32,
}

impl Default for StageConfig {
    fn default() -> Self {
        Self { stage: 1 }
    }
}

impl StageConfig {
    pub fn reset_to_first_stage(&mut self) {
        self.stage = 1;
    }

    pub fn advance(&mut self) {
        self.stage = self.stage.saturating_add(1).max(1);
    }
}
