use bevy::prelude::*;
use bevy::log;

use crate::ui::{Ui, UiAssets};
use crate::ui::plugins::hud::Hud;

pub fn spawn_hud(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
) {
    let hud = build_hud(commands, ui_assets);
}

pub fn despawn_hud(
    mut commands: Commands,
    hud: Single<Entity, With<Hud>>,
) {
    #[cfg(feature = "debug")]
    log::info!("Despawning Hud");
    commands.entity(*hud).despawn();
}

pub fn build_hud(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
) -> Entity {
    return commands.spawn(
        (
            Name::new("Hud"),
            Ui,
            Hud,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.9, 0.0, 0.5)),
            GlobalTransform::default(),
            GlobalZIndex(2),
            children![
                (
                    Text::new("Hud"),
                    TextFont {
                        font_size: 14.0,
                        font: ui_assets.font.clone(),
                        ..Default::default()
                    },
                    TextColor(Color::BLACK),
                    TextLayout {
                        justify: Justify::Center,
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 1.)
                )
            ]
        )
    )
    .id();
}