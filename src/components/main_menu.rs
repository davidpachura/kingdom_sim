use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub enum MainMenuAction{
    NewGame,
    Quit
}