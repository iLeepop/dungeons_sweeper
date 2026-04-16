use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;

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

    app.add_systems(Startup, (setup_game).chain());

    app.insert_state(AppState::OutGame);

    app.add_plugins(DungeonsPlugin {});

    app.run();
}

fn setup_game(mut next_state: ResMut<NextState<AppState>>) {
    // 处理资源和设置
    next_state.set(AppState::PreGame);
}
