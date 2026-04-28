use bevy::prelude::*;

mod components;
mod events;
mod observers;
mod plugins;
pub mod resources;
mod styles;
mod systems;

pub use components::*;
pub use events::*;
pub use observers::*;
pub use plugins::*;
pub use resources::*;
pub use styles::*;
pub use systems::*;

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
        app.add_plugins(MainMenuPlugin)
            .add_plugins(GameOverMenuPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(HudPlugin);
    }
}
