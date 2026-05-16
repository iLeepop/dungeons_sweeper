use bevy::prelude::*;

use crate::character::CharacterId;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct MainMenu;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct StartGameButton;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct ContinueRunButton;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct MainMenuGemDisplay;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Reflect)]
pub struct QuitButton;

#[derive(Component)]
pub struct MainMenuCharacterPortrait;

#[derive(Component)]
pub struct MainMenuCharacterName;

#[derive(Component)]
pub struct MainMenuCharacterHint;

#[derive(Component)]
pub struct CharacterPortraitButton;

#[derive(Component, Clone, Copy)]
pub struct CharacterCarousel;

#[derive(Component, Clone, Copy)]
pub struct MainMenuCharacterOf(pub CharacterId);
