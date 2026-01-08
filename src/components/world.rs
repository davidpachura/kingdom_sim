use bevy::prelude::*;

#[derive(Component)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub squares: Vec<Square>,
}

#[derive(Component)]
pub struct Square {
    pub biome: Biome,
    pub elevation: f32,
    pub temperature: f32,
    pub moisture: f32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Biome {
    Ocean,
    Coast,
    Grassland,
    Forest,
    Desert,
    Hill,
    Mountain,
    Ice,
    Alpine,
    Snow,
    Tundra,
    BorealForest,
    Taiga,
    ColdDesert,
    TemperateForest,
    TemperateRainforest,
    HotDesert,
    Savanna,
    SubtropicalForest,
    TropicalRainforest,
}

#[derive(Component)]
pub struct BiomeDisplayUI;

