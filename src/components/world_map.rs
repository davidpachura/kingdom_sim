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
}

pub enum Biome {
    Ocean,
    Coast,
    Grassland,
    Forest,
    Desert,
}

