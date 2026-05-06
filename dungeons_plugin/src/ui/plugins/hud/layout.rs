use bevy::log;
use bevy::prelude::*;

use crate::ui::plugins::hud::{HPBar, Hud, HPProgressBar};
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
                    Text::new("HP"),
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
                    Transform::from_xyz(0., 0., 1.),
                    HPBar,
                ),
                (
                    HPProgressBar,
                    Sprite {
                        rect: Some(Rect { min: Vec2::new(10., 20.), max: Vec2::new(10., 20.), }),
                        color: Color::srgb(0.9, 0.0, 0.0),
                        ..Default::default()
                    },
                    Transform {
                        translation: Vec3::new(0., 0., 1.),
                        rotation: Quat::from_rotation_z(0.),
                        scale: Vec3::new(1., 1., 1.),
                    }
                )
            ],
        ))
        .id();
}
