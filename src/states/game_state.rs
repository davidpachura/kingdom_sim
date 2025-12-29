#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    WorldGenSetup,
    WorldGenerating,
    Playing,
}