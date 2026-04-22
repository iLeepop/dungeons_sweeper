use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;

use dungeons_plugin::{AppState, DungeonsPlugin};
use dungeons_plugin::resources::board_option::{BoardOption, TileSize};

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

    app.add_systems(Startup, (setup_game, setup_board_options).chain());

    app.insert_state(AppState::OutGame);

    app.add_plugins(DungeonsPlugin {});

    app.run();
}

fn setup_game(mut next_state: ResMut<NextState<AppState>>) {
    // 处理资源和设置
    next_state.set(AppState::PreGame);
}

fn setup_board_options(mut commands: Commands) {
    commands.insert_resource(BoardOption {
        map_size: (50, 50),
        tile_size: TileSize { width: 35, height: 35 },
        padding: 1,
        monster_count: 100,
        treasure_count: 30,
    });
}
