use bevy::log;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::character::{character_def, CharacterId, SelectedCharacter, UnlockedCharacters};
use crate::components::Gem;
use crate::save::{GlobalProfile, RunSaveAvailable};
use crate::ui::UiAssets;
use crate::ui::{
    Ui,
    plugins::main_menu::components::{
        CharacterCarousel, CharacterPortraitButton, ContinueRunButton, MainMenu,
        MainMenuCharacterHint, MainMenuCharacterName, MainMenuCharacterPortrait, MainMenuGemDisplay,
        QuitButton, StartGameButton,
    },
};

pub fn spawn_main_menu(
    commands: Commands,
    ui_assets: Res<UiAssets>,
    global_gem: Single<&Gem, With<GlobalProfile>>,
    run_available: Res<RunSaveAvailable>,
    selected: Res<SelectedCharacter>,
    unlocked: Res<UnlockedCharacters>,
) {
    let _main_menu = build_main_menu(
        commands,
        ui_assets,
        global_gem.0,
        run_available.0,
        selected.id,
        unlocked.is_unlocked(selected.id),
    );
}

pub fn despawn_main_menu(mut commands: Commands, main_menu: Single<Entity, With<MainMenu>>) {
    #[cfg(feature = "debug")]
    log::info!("Despawning main menu");
    commands.entity(*main_menu).despawn();
}

fn portrait_brightness(unlocked: bool) -> f32 {
    if unlocked {
        1.0
    } else {
        0.35
    }
}

pub fn build_main_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    global_gems: u32,
    can_continue: bool,
    selected_id: CharacterId,
    selected_unlocked: bool,
) -> Entity {
    let def = character_def(selected_id);
    let unlocked = selected_unlocked;
    let bright = portrait_brightness(unlocked);
    let portrait_color = Color::srgb(
        def.portrait_color.to_srgba().red * bright,
        def.portrait_color.to_srgba().green * bright,
        def.portrait_color.to_srgba().blue * bright,
    );

    let (start_bg, start_text) = button_style_for_unlocked(selected_unlocked);
    let continue_enabled = can_continue && selected_unlocked;
    let (continue_bg, continue_text) = if continue_enabled {
        (
            tailwind::SLATE_500.into(),
            Color::WHITE,
        )
    } else {
        (
            tailwind::SLATE_700.into(),
            Color::srgb(0.7, 0.7, 0.7),
        )
    };

    let hint = if unlocked {
        "点击确认选择".to_string()
    } else {
        format!(
            "未解锁 — 点击花费 {} 宝石解锁",
            def.unlock_cost.unwrap_or(0)
        )
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
                        width: Val::Px(320.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..Default::default()
                    },
                    CharacterCarousel,
                    children![
                        (
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(120.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Pickable::IGNORE,
                            children![(
                                Text::new("← E"),
                                TextFont {
                                    font_size: 22.0,
                                    font: ui_assets.font.clone(),
                                    ..Default::default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                                TextLayout {
                                    justify: Justify::Center,
                                    ..Default::default()
                                },
                            )]
                        ),
                        (
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(6.0),
                                ..Default::default()
                            },
                            children![
                                (
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(120.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    Button,
                                    CharacterPortraitButton,
                                    MainMenuCharacterPortrait,
                                    BackgroundColor(portrait_color),
                                ),
                                (
                                    MainMenuCharacterName,
                                    Text::new(def.display_name),
                                    TextFont {
                                        font_size: 22.0,
                                        font: ui_assets.font.clone(),
                                        ..Default::default()
                                    },
                                    TextColor(Color::WHITE),
                                ),
                                (
                                    MainMenuCharacterHint,
                                    Text::new(hint),
                                    TextFont {
                                        font_size: 14.0,
                                        font: ui_assets.font.clone(),
                                        ..Default::default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.85, 0.7)),
                                ),
                            ]
                        ),
                        (
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(120.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Pickable::IGNORE,
                            children![(
                                Text::new("Q →"),
                                TextFont {
                                    font_size: 22.0,
                                    font: ui_assets.font.clone(),
                                    ..Default::default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                                TextLayout {
                                    justify: Justify::Center,
                                    ..Default::default()
                                },
                            )]
                        ),
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
                    BackgroundColor(start_bg),
                    GlobalTransform::default(),
                    StartGameButton,
                    children![(
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 20.0,
                            font: ui_assets.font.clone(),
                            ..Default::default()
                        },
                        TextColor(start_text),
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
                        TextColor(continue_text),
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

fn button_style_for_unlocked(unlocked: bool) -> (Color, Color) {
    if unlocked {
        (tailwind::SLATE_500.into(), Color::WHITE)
    } else {
        (tailwind::SLATE_700.into(), Color::srgb(0.7, 0.7, 0.7))
    }
}
