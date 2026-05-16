use bevy::log;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::components::Gem;
use crate::save::{GlobalProfile, RunSaveAvailable};
use crate::ui::UiAssets;
use crate::ui::{
    Ui,
    plugins::main_menu::components::{
        ContinueRunButton, MainMenu, MainMenuGemDisplay, QuitButton, StartGameButton,
    },
};

pub fn spawn_main_menu(
    commands: Commands,
    ui_assets: Res<UiAssets>,
    global_gem: Single<&Gem, With<GlobalProfile>>,
    run_available: Res<RunSaveAvailable>,
) {
    let _main_menu = build_main_menu(commands, ui_assets, global_gem.0, run_available.0);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu: Single<Entity, With<MainMenu>>) {
    #[cfg(feature = "debug")]
    log::info!("Despawning main menu");
    commands.entity(*main_menu).despawn();
}

pub fn build_main_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    global_gems: u32,
    can_continue: bool,
) -> Entity {
    let continue_bg: Color = if can_continue {
        tailwind::SLATE_500.into()
    } else {
        tailwind::SLATE_700.into()
    };

    let continue_text_color: Color = if can_continue {
        Color::WHITE
    } else {
        Color::srgb(0.7, 0.7, 0.7)
    };

    return commands
        .spawn((
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
                    children![(
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
                    )]
                ),
                (
                    Node {
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    // 纯展示：不阻挡下方 Start/Continue/Quit 的 picking（Bevy 0.18 默认会 block lower）。
                    Pickable::IGNORE,
                    MainMenuGemDisplay,
                    children![(
                        Text::new(format!("宝石: {}", global_gems)),
                        TextFont {
                            font_size: 24.0,
                            font: ui_assets.font.clone(),
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: Justify::Center,
                            ..Default::default()
                        },
                    )],
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
                    children![(
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
                    BackgroundColor(continue_bg),
                    GlobalTransform::default(),
                    ContinueRunButton,
                    children![(
                        Text::new("Continue"),
                        TextFont {
                            font_size: 20.0,
                            font: ui_assets.font.clone(),
                            ..Default::default()
                        },
                        TextColor(continue_text_color),
                        TextLayout {
                            justify: Justify::Center,
                            ..Default::default()
                        },
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
                    QuitButton,
                    children![(
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
                    )]
                ),
            ],
        ))
        .id();
}
