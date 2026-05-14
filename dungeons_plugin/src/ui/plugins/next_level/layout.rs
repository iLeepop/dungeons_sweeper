use bevy::{color::palettes::tailwind, prelude::*};

use crate::ui::UiAssets;
use crate::ui::{
    Ui,
    plugins::next_level::components::{
        NextLevelContinueButton, NextLevelMenu, NextLevelQuitMainMenuButton,
    },
};

pub fn spawn_next_level_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let _ = build_next_level_menu(&mut commands, ui_assets.as_ref());
}

pub fn despawn_next_level_menu(mut commands: Commands, menu: Single<Entity, With<NextLevelMenu>>) {
    commands.entity(*menu).despawn();
}

fn build_next_level_menu(commands: &mut Commands, ui_assets: &UiAssets) -> Entity {
    commands
        .spawn((
            Name::new("NextLevelMenu"),
            Ui,
            NextLevelMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.05, 0.15, 0.25, 0.65)),
            GlobalTransform::default(),
            GlobalZIndex(4),
            children![
                (
                    Text::new("关卡完成"),
                    TextFont {
                        font_size: 38.0,
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
                    Text::new("进入下一关？"),
                    TextFont {
                        font_size: 22.0,
                        font: ui_assets.font.clone(),
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.85, 0.9, 0.95)),
                    TextLayout {
                        justify: Justify::Center,
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 1.)
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
                    BackgroundColor(tailwind::EMERALD_600.into()),
                    GlobalTransform::default(),
                    NextLevelContinueButton,
                    children![(
                        Text::new("Continue"),
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
                    NextLevelQuitMainMenuButton,
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
