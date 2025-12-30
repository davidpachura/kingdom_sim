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

#[derive(Component)]
pub struct SeedField;

#[derive(Component)]
pub enum GameConfigAction {
    Generate,
    Back,
}

