mod components;
pub mod resources;
mod systems;
mod events;
mod utils;
mod observers;
mod bundles;
pub mod ui;

use std::collections::HashMap;

use bevy::ecs::system::ObserverSystem;
use bevy::prelude::*;

use crate::resources::enemy_assets::EnemyAssets;
use crate::resources::tile_map::TileMap;
use crate::resources::tile::Tile;
use crate::resources::board::Board;
use crate::resources::view2d::View2d;
use crate::components::coordinates::Coordinates;
use crate::components::view::View;
use crate::bundles::{cover, enemy_bundle, enemy_neighbor_bundle, grass_bundle, item_bundle, out_way_bundle, player_bundle, safe_bundle};
use crate::resources::board_option::{BoardOption, TileSize};
use crate::ui::GameUIPlugin;
use crate::utils::bounds::Bounds2;
use crate::observers::{enemy_havier_handler, player_action, taggle_consumer, view_move_consumer};
use crate::systems::{input_handler, keyboard_input_handler, uncover_tile};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Default,
    MainMenu,
    PreGame,
    InGame,
    RestartGame,
    BackMainMenu,
    GamePause,
    GameOver,
}

pub struct DungeonsPlugin {}

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUIPlugin);
        app.add_systems(Startup, Self::setup_camera)
        .add_systems(
            OnEnter(AppState::PreGame),
            (
                        Self::setup_player,
                        Self::create_board
                    ).chain()
        )
        .add_systems(
            Update,
            (
                input_handler,
                keyboard_input_handler,
            ).chain().run_if(in_state(AppState::InGame))
        )
        .add_systems(
            PostUpdate,
            uncover_tile.run_if(in_state(AppState::InGame))
        );

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

    pub fn setup_camera(
        mut commands: Commands,
    ) {
        let camera = commands.spawn((
            Camera2d,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            View {
                speed: 10,
            },
        )).id();
        commands.insert_resource(View2d {
            camera,
            position: Vec3::new(0.0, 0.0, 0.0),
        });
    }

    pub fn setup_player(
        mut commands: Commands,
    ) {
        commands.spawn(
            player_bundle(),
        );
    }

    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOption>,
        enemy_assets: Res<EnemyAssets>,
        mut next_state: ResMut<NextState<AppState>>,
    ) {
        let mut tile_map = TileMap::new(board_options.map_size.0, board_options.map_size.1);
        tile_map.set_additem(board_options.safe_count, board_options.out_way_count, board_options.monster_count, board_options.treasure_count);
        let mut tile_map = TileMap::new(board_options.map_size.0, board_options.map_size.1);
        tile_map.set_additem(board_options.safe_count, board_options.out_way_count, board_options.monster_count, board_options.treasure_count);
        #[cfg(feature = "debug")]
        println!("{}", tile_map.console_output());

        let board_position = Vec3::new(0.0, 0.0, 0.0);

        let tile_size = board_options.tile_size();

        let padding = board_options.padding();

        let board_size = Vec3::new((tile_map.width() * tile_size.width) as f32, (tile_map.height() * tile_size.height) as f32, 0.0);

        let mut tiles = HashMap::new();

        let mut covers = HashMap::new();

        let board_entity = commands.spawn((
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
            Self::spawn_tiles(
                parent, 
                &tile_map, 
                tile_size, 
                padding, 
                &board_options.counter_font,
                &mut tiles, 
                &mut covers,
                board_size,
                &enemy_assets,
            );
        })
        .id();

        commands.insert_resource(Board {
            tile_map,
            tile_size: tile_size,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size.xy(),
            },
            tiles,
            covers,
            board_entity: Some(board_entity),
        });

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
    ) {
        for (y, line) in tile_map.tiles().iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coord = Coordinates { x: x as u32, y: y as u32 };
                match tile {
                    Tile::Safe => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                safe_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size, 
                                )
                            )
                            .id()
                        );
                    }
                    Tile::OutWay => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                out_way_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size,
                                )
                            )
                            .with_children(|parent| {
                                let cover = parent.spawn(
                                    cover(tile_size, padding)
                                ).id();
                                covers.insert(coord, cover);
                            })
                            .id()
                        );
                    },
                    Tile::Grass => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                grass_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size, 
                                )
                            )
                            .with_children(|parent| {
                                let cover = parent.spawn(
                                    cover(tile_size, padding)
                                ).id();
                                covers.insert(coord, cover);
                            })
                            .id()
                        );
                    },
                    Tile::Enemy(enemy_type) => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                enemy_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size, 
                                    &enemy_assets,
                                    *enemy_type,
                                )
                            )
                            .with_children(|parent| {
                                let cover = parent.spawn(
                                    cover(tile_size, padding)
                                ).id();
                                covers.insert(coord, cover);
                            })
                            .id()
                        );
                    },
                    Tile::EnemyNeighbor(count) => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                enemy_neighbor_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size, 
                                    *count,
                                    counter_font,
                                )
                            )
                            .with_children(|parent| {
                                let cover = parent.spawn(
                                    cover(tile_size, padding)
                                ).id();
                                covers.insert(coord, cover);
                            })
                            .id()
                        );
                    },
                    Tile::Treasure => {
                        tiles.insert(
                            coord, 
                            commands.spawn(
                                item_bundle(
                                    coord, 
                                    tile_size, 
                                    padding, 
                                    board_size, 
                                )
                            )
                            .with_children(|parent| {
                                let cover = parent.spawn(
                                    cover(tile_size, padding)
                                ).id();
                                covers.insert(coord, cover);
                            })
                            .id()
                        );
                    },
                }
            }
        }
    }
}

fn set_board_observer(
    app: &mut App,
) {
    app
    .add_observer(
        taggle_consumer,
    )
    .add_observer(
        view_move_consumer,
    )
    .add_observer(
        enemy_havier_handler,
    )
    .add_observer(
        player_action,
    );
}
