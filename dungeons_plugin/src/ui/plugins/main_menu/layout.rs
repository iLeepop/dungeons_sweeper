use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*, color::palettes::tailwind};
use bevy::log;

use crate::ui::{Ui, plugins::main_menu::components::{MainMenu, StartGameButton, QuitButton}};
use crate::ui::UiAssets;

pub fn spawn_main_menu(
    commands: Commands,
    ui_assets: Res<UiAssets>,
) {
    let _main_menu = build_main_menu(commands, ui_assets);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu: Single<Entity, With<MainMenu>>
) {
    #[cfg(feature = "debug")]
    log::info!("Despawning main menu");
    commands.entity(*main_menu).despawn();
}

pub fn build_main_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
) -> Entity {
    return commands.spawn(
        (
            Name::new("MainMenu"),
            Ui,
            MainMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.4, 0.9)),
            GlobalTransform::default(),
            Transform::from_xyz(0., 0., 0.),
            children![
                (
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(120.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    GlobalTransform::default(),
                    children![
                        (
                            Text::new("Dungeons Sweeper"),
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
                        )
                    ]
                ),
                (
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    StartGameButton,
                    children![
                        (
                            Text::new("Start Game"),
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
                        )
                    ]
                ),
                (
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    QuitButton,
                    children![
                        (
                            Text::new("Quit"),
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
                        )
                    ]
                ),
            ]
        )
    ).id();
}