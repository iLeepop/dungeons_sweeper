use bevy::log;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::ui::UiAssets;
use crate::ui::{
    Ui,
    plugins::pause_menu::components::{PauseMenu, QuitMainMenuButton, RestartButton, ResumeButton},
};

pub fn spawn_pause_menu(commands: Commands, ui_assets: Res<UiAssets>) {
    let _pause_menu = build_pause_menu(commands, ui_assets);
}

pub fn despawn_pause_menu(mut commands: Commands, pause_menu: Single<Entity, With<PauseMenu>>) {
    #[cfg(feature = "debug")]
    log::info!("Despawning pause menu");
    commands.entity(*pause_menu).despawn();
}

pub fn build_pause_menu(mut commands: Commands, ui_assets: Res<UiAssets>) -> Entity {
    return commands
        .spawn((
            Name::new("PauseMenu"),
            Ui,
            PauseMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.1, 0.4, 0.9, 0.5)),
            GlobalTransform::default(),
            GlobalZIndex(3),
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
                    children![(
                        Text::new("Pause Menu"),
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
                    )]
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
                    RestartButton,
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
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    ResumeButton,
                    children![(
                        Text::new("Resume"),
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
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(tailwind::SLATE_500.into()),
                    GlobalTransform::default(),
                    QuitMainMenuButton,
                    children![(
                        Text::new("Quit to Main Menu"),
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
            ],
        ))
        .id();
}
