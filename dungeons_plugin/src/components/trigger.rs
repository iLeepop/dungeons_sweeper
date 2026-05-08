use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Reflect)]
pub enum RemainingType {
    UnLimit,
    Limit(u8),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct TriggerRemaining(pub RemainingType);

impl TriggerRemaining {
    pub fn new(times: u8) -> Self {
        TriggerRemaining(RemainingType::Limit(times))
    }

    pub fn zero() -> Self {
        TriggerRemaining(RemainingType::Limit(0))
    }

    pub fn unlimit() -> Self {
        TriggerRemaining(RemainingType::UnLimit)
    }

    /// 尝试消耗一次
    /// 如果剩余次数为0，则返回false
    /// 如果剩余次数为无限，则返回true
    /// 如果剩余次数为有限，则消耗一次，并返回true
    /// 如果剩余次数为有限，且剩余次数为0，则返回false
    pub fn try_consume_one(&mut self) -> bool {
        let r = self.0;
        match r {
            RemainingType::UnLimit => true,
            RemainingType::Limit(mut t) => {
                if t > 0 {
                    t -= 1;
                    self.0 = RemainingType::Limit(t);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Default for TriggerRemaining {
    fn default() -> Self {
        TriggerRemaining(RemainingType::Limit(1))
    }
}
