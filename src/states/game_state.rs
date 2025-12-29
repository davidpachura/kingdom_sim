use bevy::state::state::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    WorldGenSetup,
    WorldGenerating,
    Playing,
}