use bevy::prelude::*;
use bevy::log;

use crate::ui::plugins::hud::{HPBar, Hud};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Message, Reflect)]
pub struct HPBarChangeMessage {
    pub hp: i8,
    pub max_hp: i8,
}

pub fn change_hp_bar(
    mut hp_reader: MessageReader<HPBarChangeMessage>,
    mut hud: Single<&mut Text, With<HPBar>>
) {
    hp_reader.read().for_each(| msg | {
        #[cfg(feature = "debug")]
        log::info!("HPBarChangeMessage: {:?}", msg);
        hud.0 = format!("HP: {}/{}", msg.hp, msg.max_hp);
    });
}