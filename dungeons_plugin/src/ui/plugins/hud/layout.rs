use bevy::log;
use bevy::prelude::*;

use crate::ui::plugins::hud::{HPBar, HPProgressBar, Hud};
use crate::ui::{Ui, UiAssets};

pub fn spawn_hud(commands: Commands, ui_assets: Res<UiAssets>) {
    let _hud = build_hud(commands, ui_assets);
}

pub fn despawn_hud(mut commands: Commands, hud: Single<Entity, With<Hud>>) {
    #[cfg(feature = "debug")]
    log::info!("Despawning Hud");
    commands.entity(*hud).despawn();
}

pub fn build_hud(mut commands: Commands, ui_assets: Res<UiAssets>) -> Entity {
    return commands
        .spawn((
            Name::new("Hud"),
            Ui,
            Hud,
            Node {
                width: Val::Px(170.0),
                height: Val::Px(118.0),
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                position_type: PositionType::Relative,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.9, 0.0, 0.5)),
            GlobalTransform::default(),
            GlobalZIndex(2),
            children![
                (
                    Text::new("血量: —\n护盾: —\n攻击力: —\n金币: —\n宝石: —"),
                    TextFont {
                        font_size: 12.0,
                        font: ui_assets.font.clone(),
                        ..Default::default()
                    },
                    TextColor(Color::BLACK),
                    TextLayout {
                        justify: Justify::Center,
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 1.),
                    HPBar,
                ),
                (
                    HPProgressBar,
                    Sprite {
                        color: Color::srgb(0.85, 0.1, 0.1),
                        custom_size: Some(Vec2::new(100.0, 6.0)),
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 1.),
                )
            ],
        ))
        .id();
}
