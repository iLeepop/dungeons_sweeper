use bevy::render::render_resource::Texture;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;

use std::collections::HashMap;

use dungeons_plugin::{AppState, DungeonsPlugin};
use dungeons_plugin::resources::board_option::{BoardOption, TileSize};
use dungeons_plugin::resources::enemy_assets::EnemyAssets;
use dungeons_plugin::resources::tile::EnemyType;

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

fn setup_board_options(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let counter_font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
    // 设置地图大小、瓷砖大小、瓷砖间距、怪物数量、宝藏数量
    commands.insert_resource(BoardOption {
        map_size: (5, 5),
        tile_size: TileSize { width: 35, height: 35 },
        padding: 1,
        counter_font: counter_font,
        monster_count: 1,
        treasure_count: 10,
    });

    // 加载资源
    let enemys_texture: Handle<Image> = asset_server.load("sprites/enemys.png");
    let enemys_layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 5, 3, None, None);
    let enemys_texture_atlas_layout = textures_atlas_layouts.add(enemys_layout);
    
    let mut enemy_types = HashMap::new();
    enemy_types.insert(EnemyType::Eye, 0);
    enemy_types.insert(EnemyType::MagicEye, 1);
    enemy_types.insert(EnemyType::Swamp, 2);
    enemy_types.insert(EnemyType::BlueGiant, 3);
    enemy_types.insert(EnemyType::RedGiant, 4);
    enemy_types.insert(EnemyType::Yeti, 5);
    enemy_types.insert(EnemyType::EliteYeti, 6);
    enemy_types.insert(EnemyType::Cyclops, 7);
    enemy_types.insert(EnemyType::Gonin, 8);
    enemy_types.insert(EnemyType::DoubleGonin, 9);
    enemy_types.insert(EnemyType::TinyMush, 10);
    enemy_types.insert(EnemyType::BigMush, 11);
    enemy_types.insert(EnemyType::MushMan, 12);
    enemy_types.insert(EnemyType::Slim, 13);
    enemy_types.insert(EnemyType::EliteSlim, 14);
    // 插入资源
    commands.insert_resource(EnemyAssets {
        texture: enemys_texture,
        atlas_layout: enemys_texture_atlas_layout,
        enemy_atlas_layout: enemy_types,
    });
}
