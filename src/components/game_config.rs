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
pub struct TerrainScaleField;

#[derive(Component)]
pub struct ContinentalScaleField;

#[derive(Component)]
pub struct OctaveField;

#[derive(Component)]
pub struct SeaThresholdField;

#[derive(Component)]
pub struct MountainThresholdField;

#[derive(Component)]
pub struct ScalingFactorField;

#[derive(Component)]
pub enum GameConfigAction {
    Generate,
    Back,
}

