use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{Coordinates, Enemy, Exposed, Uncover};
use crate::components::{Damage, Defense, Gem, GoldCoin, Health, Player};
use crate::resources::board::Board;
use crate::resources::board_option::BoardOption;
use crate::resources::enemy_type::EnemyType;
use crate::resources::tile::Tile;
use crate::resources::tile_map::TileMap;
use crate::resources::StageConfig;
use crate::resources::view2d::View2d;
use crate::character::CharacterId;
use crate::effects::{capture_effect_specs, ActiveEffectSpecs, SerializableEffect};
use crate::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializableTile {
    Spawn,
    Grass,
    Enemy(u8),
    EnemyNeighbor(u16),
    Treasure,
    OutWay,
    Safe,
}

impl From<&Tile> for SerializableTile {
    fn from(tile: &Tile) -> Self {
        match tile {
            Tile::Spawn => SerializableTile::Spawn,
            Tile::Grass => SerializableTile::Grass,
            Tile::Enemy(t) => SerializableTile::Enemy(t.discriminant_index() as u8),
            Tile::EnemyNeighbor(n) => SerializableTile::EnemyNeighbor(*n),
            Tile::Treasure => SerializableTile::Treasure,
            Tile::OutWay => SerializableTile::OutWay,
            Tile::Safe => SerializableTile::Safe,
        }
    }
}

impl From<SerializableTile> for Tile {
    fn from(st: SerializableTile) -> Self {
        match st {
            SerializableTile::Spawn => Tile::Spawn,
            SerializableTile::Grass => Tile::Grass,
            SerializableTile::Enemy(i) => {
                Tile::Enemy(EnemyType::from_discriminant_index(i as usize))
            }
            SerializableTile::EnemyNeighbor(n) => Tile::EnemyNeighbor(n),
            SerializableTile::Treasure => Tile::Treasure,
            SerializableTile::OutWay => Tile::OutWay,
            SerializableTile::Safe => Tile::Safe,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSnapshot {
    pub map_size: (u32, u32),
    pub difficulty_factor: f32,
    pub safe_count: u16,
    pub out_way_count: u16,
    pub monster_count: u16,
    pub treasure_count: u16,
    pub tiles: Vec<Vec<SerializableTile>>,
    pub uncovered: Vec<(u32, u32)>,
    pub enemy_hp: Vec<((u32, u32), i8)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub health: i8,
    pub damage: u8,
    pub defense: i8,
    pub gold: u32,
    pub gems: u32,
    #[serde(default)]
    pub character_id: u8,
    #[serde(default)]
    pub effect_specs: Vec<SerializableEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunPauseKind {
    InGame,
    NextLevel,
}

pub fn tile_type_had_cover(tile: &Tile) -> bool {
    !matches!(tile, Tile::Spawn)
}

pub fn board_snapshot_from_board(
    board: &Board,
    board_options: &BoardOption,
    enemy_health: &Query<&Health, With<Enemy>>,
) -> BoardSnapshot {
    let width = board.tile_map.width();
    let height = board.tile_map.height();
    let mut tiles = Vec::with_capacity(height as usize);
    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let coord = Coordinates { x, y };
            let tile = board
                .tile_map
                .get_tile(coord)
                .expect("in-bounds tile");
            row.push(SerializableTile::from(tile));
        }
        tiles.push(row);
    }

    let mut uncovered = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let coord = Coordinates { x, y };
            let Some(tile) = board.tile_map.get_tile(coord) else {
                continue;
            };
            if tile_type_had_cover(tile) && !board.covers.contains_key(&coord) {
                uncovered.push((x, y));
            }
        }
    }

    let mut enemy_hp = Vec::new();
    for (coord, entity) in &board.tiles {
        let Some(tile) = board.tile_map.get_tile(*coord) else {
            continue;
        };
        if matches!(tile, Tile::Enemy(_)) {
            if let Ok(h) = enemy_health.get(*entity) {
                enemy_hp.push(((coord.x, coord.y), h.0));
            }
        }
    }

    BoardSnapshot {
        map_size: board_options.map_size,
        difficulty_factor: board_options.difficulty_factor,
        safe_count: board_options.safe_count,
        out_way_count: board_options.out_way_count,
        monster_count: board_options.monster_count,
        treasure_count: board_options.treasure_count,
        tiles,
        uncovered,
        enemy_hp,
    }
}

pub fn tile_map_from_snapshot(snapshot: &BoardSnapshot) -> TileMap {
    let grid: Vec<Vec<Tile>> = snapshot
        .tiles
        .iter()
        .map(|row| row.iter().map(|t| Tile::from(*t)).collect())
        .collect();
    TileMap::from_saved_grid(
        snapshot.map_size.0,
        snapshot.map_size.1,
        snapshot.difficulty_factor,
        grid,
    )
}

pub fn apply_board_option_from_snapshot(board_options: &mut BoardOption, snapshot: &BoardSnapshot) {
    board_options.map_size = snapshot.map_size;
    board_options.difficulty_factor = snapshot.difficulty_factor;
    board_options.safe_count = snapshot.safe_count;
    board_options.out_way_count = snapshot.out_way_count;
    board_options.monster_count = snapshot.monster_count;
    board_options.treasure_count = snapshot.treasure_count;
}

pub fn apply_board_restoration(
    commands: &mut Commands,
    board: &mut Board,
    snapshot: &BoardSnapshot,
    enemy_health: &mut Query<&mut Health, With<Enemy>>,
) {
    for (x, y) in &snapshot.uncovered {
        let coord = Coordinates { x: *x, y: *y };
        let Some(cover_entity) = board.covers.remove(&coord) else {
            continue;
        };
        let Some(tile_entity) = board.tiles.get(&coord).copied() else {
            continue;
        };
        commands.entity(tile_entity).insert(Exposed);
        commands.entity(cover_entity).insert(Uncover);
    }

    for ((x, y), hp) in &snapshot.enemy_hp {
        let coord = Coordinates { x: *x, y: *y };
        let Some(entity) = board.tiles.get(&coord).copied() else {
            continue;
        };
        if let Ok(mut health) = enemy_health.get_mut(entity) {
            health.0 = *hp;
        }
    }
}

pub fn pause_kind_from_state(state: &AppState) -> RunPauseKind {
    match state {
        AppState::NextLevel => RunPauseKind::NextLevel,
        AppState::GamePause | AppState::InGame => RunPauseKind::InGame,
        _ => RunPauseKind::InGame,
    }
}

pub fn app_state_from_pause_kind(kind: RunPauseKind) -> AppState {
    match kind {
        RunPauseKind::InGame => AppState::InGame,
        RunPauseKind::NextLevel => AppState::NextLevel,
    }
}

pub fn capture_player_snapshot(
    health: &Health,
    damage: &Damage,
    defense: &Defense,
    gold: &GoldCoin,
    gems: &Gem,
    character_id: CharacterId,
    active_effects: &ActiveEffectSpecs,
) -> PlayerSnapshot {
    PlayerSnapshot {
        health: health.0,
        damage: damage.0,
        defense: defense.0,
        gold: gold.0,
        gems: gems.0,
        character_id: character_id.to_index(),
        effect_specs: capture_effect_specs(active_effects),
    }
}

pub fn character_id_from_snapshot(snap: &PlayerSnapshot) -> CharacterId {
    CharacterId::from_index(snap.character_id).unwrap_or(CharacterId::Herbalist)
}

pub fn apply_player_snapshot(
    health: &mut Health,
    damage: &mut Damage,
    defense: &mut Defense,
    gold: &mut GoldCoin,
    gems: &mut Gem,
    snap: &PlayerSnapshot,
) {
    health.0 = snap.health;
    damage.0 = snap.damage;
    defense.0 = snap.defense;
    gold.0 = snap.gold;
    gems.0 = snap.gems;
}

pub fn restore_view(view2d: &mut View2d, view_transform: &mut Transform, position: Vec3) {
    view_transform.translation = position;
    view2d.position = position;
}
