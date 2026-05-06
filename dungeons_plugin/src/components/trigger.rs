use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct TriggerTimes(pub u8);

impl TriggerTimes {
    pub fn new(times: u8) -> Self {
        TriggerTimes(times)
    }

    pub fn zero() -> Self {
        TriggerTimes(0)
    }
}

impl Default for TriggerTimes {
    fn default() -> Self {
        TriggerTimes(1)
    }
}
