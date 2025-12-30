use bevy::prelude::*;

#[derive(Component)]
pub struct TextInput;

#[derive(Component)]
pub struct Focused;

#[derive(Component)]
pub struct InputValue {
    pub text: String,
}

#[derive(Component)]
pub struct GameConfigUI;

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TextField {
    Seed,
}

#[derive(Component)]
pub enum GameConfigAction {
    Generate,
    Back,
}

