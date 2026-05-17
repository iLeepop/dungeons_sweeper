use bevy::color::palettes::tailwind;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use crate::character::{
    SelectedCharacter, UnlockedCharacters, character_def, persist_character_selection,
    try_unlock_character,
};
use crate::components::Gem;
use crate::save::{GlobalProfile, RunSaveAvailable, SavePaths};
use crate::ui::plugins::main_menu::components::{
    CharacterPortraitButton, ContinueRunButton, MainMenuCharacterHint, MainMenuCharacterName,
    MainMenuCharacterPortrait, MainMenuGemDisplay, StartGameButton,
};

/// Q 向右、E 向左切换角色。
pub fn keyboard_switch_main_menu_character(
    input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedCharacter>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        selected.id = selected.id.next();
    }
    if input.just_pressed(KeyCode::KeyE) {
        selected.id = selected.id.prev();
    }
}

pub fn interact_with_character_portrait(
    mut portrait: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CharacterPortraitButton>),
    >,
    mut selected: ResMut<SelectedCharacter>,
    mut unlocked: ResMut<UnlockedCharacters>,
    mut global_gem: Single<&mut Gem, With<GlobalProfile>>,
    paths: Res<SavePaths>,
) {
    let (interaction, _) = match portrait.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };
    if *interaction != Interaction::Pressed {
        return;
    }
    let id = selected.id;
    if unlocked.is_unlocked(id) {
        persist_character_selection(paths.as_ref(), global_gem.0, unlocked.as_ref(), id);
        return;
    }
    let _ = try_unlock_character(
        paths.as_ref(),
        &mut global_gem,
        &mut unlocked,
        &mut selected,
        id,
    );
}

pub fn refresh_main_menu_character_display(
    selected: Res<SelectedCharacter>,
    unlocked: Res<UnlockedCharacters>,
    global_gem: Query<Ref<Gem>, With<GlobalProfile>>,
    run_available: Res<RunSaveAvailable>,
    mut menu_texts: ParamSet<(
        Query<&mut Text, With<MainMenuCharacterName>>,
        Query<&mut Text, With<MainMenuCharacterHint>>,
        Query<&mut Text, With<MainMenuGemDisplay>>,
    )>,
    mut menu_backgrounds: ParamSet<(
        Query<&mut BackgroundColor, With<MainMenuCharacterPortrait>>,
        Query<(&mut BackgroundColor, &Children, &Interaction), With<StartGameButton>>,
        Query<(&mut BackgroundColor, &Children, &Interaction), With<ContinueRunButton>>,
    )>,
    mut text_colors: Query<&mut TextColor>,
) {
    let def = character_def(selected.id);
    let is_unlocked = unlocked.is_unlocked(selected.id);
    let bright = if is_unlocked { 1.0 } else { 0.35 };
    let rgba = def.portrait_color.to_srgba();

    if selected.is_changed() || unlocked.is_changed() {
        if let Ok(mut bg) = menu_backgrounds.p0().single_mut() {
            bg.0 = Color::srgb(rgba.red * bright, rgba.green * bright, rgba.blue * bright);
        }
        if let Ok(mut t) = menu_texts.p0().single_mut() {
            **t = def.display_name.to_string();
        }
        if let Ok(mut t) = menu_texts.p1().single_mut() {
            **t = if is_unlocked {
                "点击确认选择".to_string()
            } else {
                format!(
                    "未解锁 — 点击花费 {} 宝石解锁",
                    def.unlock_cost.unwrap_or(0)
                )
            };
        }
    }
    let gem_changed = global_gem.single().map(|g| g.is_changed()).unwrap_or(false);
    if gem_changed || unlocked.is_changed() {
        if let Ok(mut t) = menu_texts.p2().single_mut() {
            if let Ok(gem) = global_gem.single() {
                **t = format!("宝石: {}", gem.0);
            }
        }
    }

    // 仅在启用状态变化时写按钮底色，避免每帧覆盖 interaction 的 Hover 颜色。
    if selected.is_changed() || unlocked.is_changed() || run_available.is_changed() {
        let start_enabled = is_unlocked;
        let continue_enabled = run_available.0 && is_unlocked;
        update_menu_button(menu_backgrounds.p1(), &mut text_colors, start_enabled);
        update_menu_button(menu_backgrounds.p2(), &mut text_colors, continue_enabled);
    }
}

fn update_menu_button<F: QueryFilter>(
    mut btn: Query<(&mut BackgroundColor, &Children, &Interaction), F>,
    text_colors: &mut Query<&mut TextColor>,
    enabled: bool,
) {
    let Ok((mut bg, children, interaction)) = btn.single_mut() else {
        return;
    };
    bg.0 = if !enabled {
        tailwind::SLATE_700.into()
    } else {
        match *interaction {
            Interaction::Pressed => tailwind::SLATE_700.into(),
            Interaction::Hovered => tailwind::SLATE_600.into(),
            Interaction::None => tailwind::SLATE_500.into(),
        }
    };
    let label_color = if enabled {
        Color::WHITE
    } else {
        Color::srgb(0.7, 0.7, 0.7)
    };
    for child in children.iter() {
        if let Ok(mut tc) = text_colors.get_mut(child) {
            tc.0 = label_color;
        }
    }
}
