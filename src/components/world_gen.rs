use bevy::prelude::*;

#[derive(Component)]
pub struct WorldData{
    pub seed: u32,
    pub terrain_scale: f64,
    pub continental_scale: f64,
    pub num_of_octaves: u32,
    pub sea_threshold: f64,
    pub mountain_threshold: f64,
}

