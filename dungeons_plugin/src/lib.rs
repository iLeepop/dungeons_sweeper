mod bundles;
mod components;
pub mod effects;
mod events;
mod observers;
pub mod resources;
mod systems;
pub mod ui;
mod utils;

use std::collections::HashMap;

use bevy::prelude::*;

use crate::bundles::{
    cover, enemy_bundle, enemy_neighbor_bundle, GrassTile, item_bundle, out_way_bundle,
    player_bundle, safe_bundle, spawn_bundle,
};
use crate::components::{Coordinates, Player, View};
use crate::effects::effect_phase_dispatch_system;
use crate::effects::EffectPhaseMessage;
use crate::effects::EffectCounters;
use crate::effects::{WorldEffectHost, WorldEffectLoader};
use crate::observers::{enemy_havier_handler, player_action, taggle_consumer, view_move_consumer};
use crate::resources::board::Board;
use crate::resources::apply_stage_to_board_option;
use crate::resources::DifficultyTuning;
use crate::resources::EnemyType;
use crate::resources::PlayerOptions;
use crate::resources::board_option::{BoardOption, TileSize};
use crate::resources::StageConfig;
use crate::resources::enemy_assets::EnemyAssets;
use crate::resources::tile::Tile;
use crate::resources::tile_map::TileMap;
use crate::resources::view2d::View2d;
use crate::systems::{input_handler, keyboard_input_handler, uncover_tile};
use crate::ui::GameUIPlugin;
use crate::ui::UiAssets;
use crate::ui::sync_player_hud_from_components;
use crate::utils::bounds::Bounds2;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Init,
    MainMenu,
    PreGame,
    InGame,
    RestartGame,
    BackMainMenu,
    GamePause,
    /// 踩出口后：选 Continue 再升关并重建棋盘（与 Esc 暂停区分）。
    NextLevel,
    GameOver,
}

// ---------------------------------------------------------------------------
// 棋盘实体与 TileMap：PreGame / 出口共用，先清理 WorldEffectHost 与旧 Board 再生成。
// ---------------------------------------------------------------------------

pub(crate) fn rebuild_board_entities(
    commands: &mut Commands,
    board_options: &BoardOption,
    enemy_assets: &EnemyAssets,
    player_options: &PlayerOptions,
    tuning: &DifficultyTuning,
    stage: u32,
    world_hosts: &Query<Entity, With<WorldEffectHost>>,
    board_entity_to_despawn: Option<Entity>,
    grass_heal: i8,
) {
    // 避免重复宿主与重复棋盘实体占用同一逻辑。
    for host_ent in world_hosts.iter() {
        commands.entity(host_ent).despawn();
    }
    if let Some(be) = board_entity_to_despawn {
        commands.entity(be).despawn();
    }
    commands.remove_resource::<Board>();

    commands.spawn((
        Name::new("WorldEffectHost"),
        WorldEffectHost,
        WorldEffectLoader::default(),
    ));

    let mut tile_map = TileMap::new(board_options.map_size.0, board_options.map_size.1);
    tile_map.set_additem(
        board_options.safe_count,
        board_options.out_way_count,
        board_options.monster_count,
        board_options.treasure_count,
        board_options.difficulty_factor,
        stage,
        player_options,
        tuning,
    );

    #[cfg(feature = "debug")]
    println!("{}", tile_map.console_output());

    let board_position = Vec3::new(0.0, 0.0, 0.0);
    let tile_size = board_options.tile_size();
    let padding = board_options.padding();
    let board_size = Vec3::new(
        (tile_map.width() * tile_size.width) as f32,
        (tile_map.height() * tile_size.height) as f32,
        0.0,
    );

    let mut tiles = HashMap::new();
    let mut covers = HashMap::new();

    let board_entity = commands
        .spawn((
            Name::new("Board"),
            Transform::from_translation(board_position),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(board_size.x as f32, board_size.y as f32)),
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            DungeonsPlugin::spawn_tiles(
                parent,
                &tile_map,
                tile_size,
                padding,
                &board_options.counter_font,
                &mut tiles,
                &mut covers,
                board_size,
                enemy_assets,
                board_options.difficulty_factor,
                grass_heal,
            );
        })
        .id();

    commands.insert_resource(Board {
        tile_map,
        tile_size,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size.xy(),
        },
        tiles,
        covers,
        board_entity: Some(board_entity),
    });
}

/// 进入下一关：升 `StageConfig`、刷新 [`BoardOption`] 计数、重建棋盘（由下一关菜单 Continue 调用）。
pub fn advance_stage_and_rebuild_board(
    commands: &mut Commands,
    board_options: &mut BoardOption,
    stage: &mut StageConfig,
    enemy_assets: &EnemyAssets,
    player_options: &PlayerOptions,
    tuning: &DifficultyTuning,
    world_hosts: &Query<Entity, With<WorldEffectHost>>,
    board_entity_to_despawn: Option<Entity>,
) {
    stage.advance();
    apply_stage_to_board_option(board_options, stage.stage);
    rebuild_board_entities(
        commands,
        board_options,
        enemy_assets,
        player_options,
        tuning,
        stage.stage,
        world_hosts,
        board_entity_to_despawn,
        player_options.grass_heal_per_trigger,
    );
}

pub struct DungeonsPlugin {}

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {

        app
        .init_state::<AppState>();

        app.add_systems(
            Startup, 
            (
                setup_game,
                setup_board_options
            ).chain()
        );

        app.add_plugins(GameUIPlugin);

        app.add_message::<EffectPhaseMessage>()
            .init_resource::<EffectCounters>()
            .add_systems(
                PostUpdate,
                effect_phase_dispatch_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                PostUpdate,
                sync_player_hud_from_components
                    .after(effect_phase_dispatch_system)
                    .run_if(
                        in_state(AppState::InGame).or(in_state(AppState::GamePause)),
                    ),
            );

        app.add_systems(Startup, Self::setup_camera)
            .add_systems(
                OnEnter(AppState::PreGame),
                (
                    apply_stage_board_options,
                    Self::setup_player,
                    Self::create_board,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (input_handler, keyboard_input_handler)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(PostUpdate, uncover_tile.run_if(in_state(AppState::InGame)));

        set_board_observer(app);
    }
}

impl Default for DungeonsPlugin {
    fn default() -> Self {
        DungeonsPlugin {}
    }
}

impl DungeonsPlugin {
    pub fn new() -> Self {
        DungeonsPlugin {}
    }

    pub fn setup_camera(mut commands: Commands) {
        let camera = commands
            .spawn((
                Camera2d,
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                View { speed: 10 },
            ))
            .id();
        commands.insert_resource(View2d {
            camera,
            position: Vec3::new(0.0, 0.0, 0.0),
        });
    }

    /// 单例玩家：重进 PreGame 时不重复生成（与全局 `Single<Player>` 查询一致）。
    pub fn setup_player(mut commands: Commands, opts: Res<PlayerOptions>, q: Query<Entity, With<Player>>) {
        if q.is_empty() {
            commands.spawn(player_bundle(opts.as_ref()));
        }
    }

    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOption>,
        enemy_assets: Res<EnemyAssets>,
        player_options: Res<PlayerOptions>,
        tuning: Res<DifficultyTuning>,
        stage: Res<StageConfig>,
        existing_board: Option<Res<Board>>,
        mut next_state: ResMut<NextState<AppState>>,
        world_hosts: Query<Entity, With<WorldEffectHost>>,
    ) {
        let board_ent = existing_board.as_ref().and_then(|b| b.board_entity);
        rebuild_board_entities(
            &mut commands,
            board_options.as_ref(),
            enemy_assets.as_ref(),
            player_options.as_ref(),
            tuning.as_ref(),
            stage.stage,
            &world_hosts,
            board_ent,
            player_options.grass_heal_per_trigger,
        );

        next_state.set(AppState::InGame);
    }

    fn spawn_tiles(
        commands: &mut ChildSpawnerCommands,
        tile_map: &TileMap,
        tile_size: TileSize,
        padding: u32,
        counter_font: &Handle<Font>,
        tiles: &mut HashMap<Coordinates, Entity>,
        covers: &mut HashMap<Coordinates, Entity>,
        board_size: Vec3,
        enemy_assets: &EnemyAssets,
        difficulty_factor: f32,
        grass_heal: i8,
    ) {
        for (y, line) in tile_map.tiles().iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coord = Coordinates {
                    x: x as u32,
                    y: y as u32,
                };
                match tile {
                    Tile::Spawn => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(spawn_bundle(coord, tile_size, padding, board_size))
                                .id(),
                        );
                    }
                    Tile::Safe => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(safe_bundle(coord, tile_size, padding, board_size))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                    Tile::OutWay => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(out_way_bundle(coord, tile_size, padding, board_size))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                    Tile::Grass => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(GrassTile::grass_bundle(
                                    coord,
                                    tile_size,
                                    padding,
                                    board_size,
                                    grass_heal,
                                ))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                    Tile::Enemy(enemy_type) => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(enemy_bundle(
                                    coord,
                                    tile_size,
                                    padding,
                                    board_size,
                                    &enemy_assets,
                                    *enemy_type,
                                    difficulty_factor,
                                ))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                    Tile::EnemyNeighbor(hp_sum) => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(enemy_neighbor_bundle(
                                    coord,
                                    tile_size,
                                    padding,
                                    board_size,
                                    u32::from(*hp_sum),
                                    counter_font,
                                ))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                    Tile::Treasure => {
                        tiles.insert(
                            coord,
                            commands
                                .spawn(item_bundle(coord, tile_size, padding, board_size))
                                .with_children(|parent| {
                                    let cover = parent.spawn(cover(tile_size, padding)).id();
                                    covers.insert(coord, cover);
                                })
                                .id(),
                        );
                    }
                }
            }
        }
    }
}

fn set_board_observer(app: &mut App) {
    app.add_observer(taggle_consumer)
        .add_observer(view_move_consumer)
        .add_observer(enemy_havier_handler)
        .add_observer(player_action);
}

fn setup_game(mut next_state: ResMut<NextState<AppState>>) {
    // 处理资源和设置
    next_state.set(AppState::MainMenu);
}

fn apply_stage_board_options(mut board: ResMut<BoardOption>, stage: Res<StageConfig>) {
    apply_stage_to_board_option(&mut board, stage.stage);
}

fn setup_board_options(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let counter_font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
    // 设置地图大小、瓷砖大小、瓷砖间距、怪物数量、宝藏数量
    commands.insert_resource(StageConfig::default());

    commands.insert_resource(BoardOption {
        map_size: (5, 5),
        tile_size: TileSize {
            width: 35,
            height: 35,
        },
        padding: 1,
        counter_font: counter_font,
        difficulty_factor: 1.0,
        safe_count: 1,
        out_way_count: 1,
        monster_count: 5,
        treasure_count: 3,
    });

    // 玩家属性与难度系数（关卡生成、HUD、草地效果共用）。
    commands.insert_resource(PlayerOptions::default());
    commands.insert_resource(DifficultyTuning::default());

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

    commands.insert_resource(UiAssets {
        font: asset_server.load("fonts/vonwaon.ttf"),
    });
}
