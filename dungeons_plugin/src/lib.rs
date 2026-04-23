mod components;
pub mod resources;
mod systems;
mod events;
mod utils;
mod observers;
mod bundles;

use std::collections::HashMap;

use bevy::{ecs::system::command, prelude::*};
use bevy::ecs::bundle::Bundle;
use bevy::log;

use crate::resources::enemy_assets::EnemyAssets;
#[cfg(feature = "debug")]
use crate::resources::tile;
use crate::resources::tile_map::TileMap;
use crate::resources::tile::Tile;
use crate::resources::board::Board;
use crate::resources::view2d::View2d;
use crate::components::coordinates::Coordinates;
use crate::components::view::View;
use crate::bundles::normal_bundle;
use crate::resources::board_option::{BoardOption, TileSize};
use crate::resources::enemy_option::EnemyOption;
use crate::utils::bounds::Bounds2;
use crate::resources::tile::EnemyType;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    PreGame,
    InGame,
    OutGame,
}

pub struct DungeonsPlugin {}

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::PreGame),
            (
                        Self::setup_camera,
                        Self::create_board
                    ).chain(),
        )
        .add_systems(
            Update,
            (
                systems::input::input_handler,
                systems::input::keyboard_input_handler,
            ).chain(),
        )
        .add_observer(
            observers::taggle_consumer::taggle_consumer,
        )
        .add_observer(
            observers::view_move_consumer::view_move_consumer,
        );
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

    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOption>,
        enemy_options: Res<EnemyOption>,
        sprites: Res<EnemyAssets>,
    ) {
        let mut tile_map = TileMap::new(board_options.map_size.0, board_options.map_size.1);
        tile_map.set_additem(board_options.monster_count, board_options.treasure_count);
        #[cfg(feature = "debug")]
        println!("{}", tile_map.console_output());

        let board_position = Vec3::new(0.0, 0.0, 0.0);

        let tile_size = board_options.tile_size();

        let padding = board_options.padding();

        let board_size = Vec3::new((tile_map.width() * tile_size.width) as f32, (tile_map.height() * tile_size.height) as f32, 0.0);

        let mut tiles = HashMap::new();

        let mut uncovers = HashMap::new();

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
                &mut tiles, 
                &mut uncovers,
                board_size,
                &sprites,
                &enemy_options,
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
            uncovers,
            board_entity: Some(board_entity),
        });
    }

    fn spawn_tiles(
        commands: &mut ChildSpawnerCommands,
        tile_map: &TileMap,
        tile_size: TileSize,
        padding: u32,
        tiles: &mut HashMap<Coordinates, Entity>,
        uncovers: &mut HashMap<Coordinates, Entity>,
        board_size: Vec3,
        sprites: &EnemyAssets,
        enemy_options: &EnemyOption,
    ) {
        for x in 0..tile_map.width() {
            for y in 0..tile_map.height() {
                let coord = Coordinates { x, y };
                tiles.insert(
                    coord, 
                    commands.spawn(     
                        normal_bundle(
                            coord, 
                            tile_size, 
                            padding, 
                            board_size, 
                            &sprites,
                            enemy_options,
                            uncovers,
                        )
                    )
                    .with_children(|parent| {
                        let cover = parent.spawn((
                            Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new((tile_size.width - padding) as f32, (tile_size.height - padding) as f32)),
                                ..Default::default()
                            },
                            Transform::from_xyz(0.0, 0.0, 2.0),
                        )).id();
                        uncovers.insert(coord, cover);
                    })
                    .id()
                );
            }
        }
    }
}
