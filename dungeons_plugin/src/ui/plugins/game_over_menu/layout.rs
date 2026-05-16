use bevy::log;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::components::{Gem, GoldCoin, Player};
use crate::save::GameOverGemsEarned;
use crate::ui::UiAssets;
use crate::ui::{
    Ui,
    plugins::game_over_menu::components::{
        GameOverMenu, GameOverQuitMainMenuButton, GameOverRestartButton,
    },
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    gold: Single<&GoldCoin, With<Player>>,
    gem: Single<&Gem, With<Player>>,
    gems_earned: Res<GameOverGemsEarned>,
) {
    let _ = build_game_over_menu(
        &mut commands,
        ui_assets.as_ref(),
        gold.0,
        gem.0,
        gems_earned.0,
    );
}

pub fn despawn_game_over_menu(mut commands: Commands, menu: Single<Entity, With<GameOverMenu>>) {
    #[cfg(feature = "debug")]
    log::info!("Despawning game over menu");
    commands.entity(*menu).despawn();
}

fn build_game_over_menu(
    commands: &mut Commands,
    ui_assets: &UiAssets,
    gold: u32,
    gems: u32,
    global_gems_earned: u32,
) -> Entity {
    commands
        .spawn((
            Name::new("GameOverMenu"),
            Ui,
            GameOverMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.2, 0.05, 0.05, 0.72)),
            GlobalTransform::default(),
            GlobalZIndex(4),
            children![
                (
                    Node {
                        width: Val::Px(420.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..Default::default()
                    },
                    GlobalTransform::default(),
                    children![
                        (
                            Text::new("Game Over"),
                            TextFont {
                                font_size: 40.0,
                                font: ui_assets.font.clone(),
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                            TextLayout {
                                justify: Justify::Center,
                                ..Default::default()
                            },
                            Transform::from_xyz(0., 0., 1.)
                        ),
                        (
                            Text::new(format!(
                                "金币: {}\n局内宝石: {}\n获得宝石: +{}",
                                gold, gems, global_gems_earned
                            )),
                            TextFont {
                                font_size: 22.0,
                                font: ui_assets.font.clone(),
                                ..Default::default()
                            },
                            TextColor(Color::srgb(0.95, 0.9, 0.75)),
                            TextLayout {
                                justify: Justify::Center,
                                ..Default::default()
                            },
                            Transform::from_xyz(0., 0., 1.)
                        ),
                    ]
                ),
                (
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    GameOverRestartButton,
                    children![(
                        Text::new("Restart"),
                        TextFont {
                            font_size: 20.0,
                            font: ui_assets.font.clone(),
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: Justify::Center,
                            ..Default::default()
                        },
                        Transform::from_xyz(0., 0., 1.)
                    )]
                ),
                (
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    GameOverQuitMainMenuButton,
                    children![(
                        Text::new("Quit to Main Menu"),
                        TextFont {
                            font_size: 18.0,
                            font: ui_assets.font.clone(),
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: Justify::Center,
                            ..Default::default()
                        },
                        Transform::from_xyz(0., 0., 1.)
                    )]
                ),
            ],
        ))
        .id()
}
