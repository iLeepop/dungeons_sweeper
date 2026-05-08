//! 单条效果条目：优先级 + 触发条件 + 行为（`dyn` 擦除具体类型）。

use crate::effects::behavior::EffectBehavior;
use crate::effects::trigger::EffectTrigger;

// ---------------------------------------------------------------------------
// EffectEntry
// ---------------------------------------------------------------------------

/// 加载器 `Vec` 中的单条记录；[`super::dispatch`] 会按 priority 与 Vec 下标做稳定排序后执行。
pub struct EffectEntry {
    /// 数值越大越先执行；同 priority 时按在 Vec 中的下标升序（先入先出）。
    pub priority: i16,
    pub trigger: EffectTrigger,
    pub(crate) effect: Box<dyn EffectBehavior>,
}

impl EffectEntry {
    /// 构造一条效果；`effect` 为任意实现了 [`EffectBehavior`] 的类型。
    pub fn new(priority: i16, trigger: EffectTrigger, effect: impl EffectBehavior + 'static) -> Self {
        Self {
            priority,
            trigger,
            effect: Box::new(effect),
        }
    }

    /// 借用内部行为（仅供调度器）。
    pub fn behavior(&self) -> &dyn EffectBehavior {
        self.effect.as_ref()
    }
}
