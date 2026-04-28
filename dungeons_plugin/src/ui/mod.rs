use bevy::prelude::*;

mod plugins;
mod systems;
mod components;
mod events;
mod observers;
mod styles;
pub mod resources;


pub use components::*;
pub use systems::*;
pub use plugins::*;
pub use events::*;
pub use observers::*;
pub use styles::*;
pub use resources::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum GameUIState {
    #[default]
    MainMenu,
    PauseMenu,
    GameOverMenu,
}

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameOverMenuPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(HudPlugin);
    }
}