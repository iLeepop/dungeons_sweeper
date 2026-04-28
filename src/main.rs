use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;

use std::collections::HashMap;

use dungeons_plugin::resources::board_option::{BoardOption, TileSize};
use dungeons_plugin::resources::enemy_assets::EnemyAssets;
use dungeons_plugin::resources::enemy_type::EnemyType;
use dungeons_plugin::ui::UiAssets;
use dungeons_plugin::{AppState, DungeonsPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Dungeons Sweeper".to_string(),
            resolution: WindowResolution::from((800, 700)),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins(EguiPlugin::default());

    app.add_plugins(DungeonsPlugin {});

    app.run();
}
