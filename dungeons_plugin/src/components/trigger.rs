use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Reflect)]
pub enum RemainingType {
    Zero,
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
        TriggerRemaining(RemainingType::Zero)
    }

    pub fn unlimit() -> Self {
        TriggerRemaining(RemainingType::UnLimit)
    }
}

impl Default for TriggerRemaining {
    fn default() -> Self {
        TriggerRemaining(RemainingType::Limit(1))
    }
}
