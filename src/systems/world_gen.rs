use std::f64::consts::PI;

use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;

use crate::components::{world::*, world_gen::WorldData};
use crate::states::game_state::GameState;

const WORLD_SIZE: i32 = 4096;
const SEA_LEVEL: f64 = 0.48;

pub fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&WorldData>,
) {
    let world_data = match query.single() {
        Ok(map) => map,
        Err(err) => {
            error!("WorldMap query failed: {:?}", err);
            return;
        }
    };
    let world_map = generate_logical_world(world_data);

    commands.spawn(world_map);

    next_state.set(GameState::Playing);
}

fn generate_logical_world(world_data: &WorldData) -> WorldMap {
    println!("Generating world");
    println!("Seed: {0}", world_data.seed);
    println!("T_Scale {0}", world_data.terrain_scale);
    println!("C_Scale {0}", world_data.continental_scale);
    println!("Temp_Scale {0}", world_data.temperature_scale);
    println!("Moist_Scale {0}", world_data.moisture_scale);
    println!("O_num: {0}", world_data.num_of_octaves);
    println!("S_Threshold {0}", world_data.sea_threshold);
    println!("Scaling_Factor {0}", world_data.scaling_factor);
    let noise_terrain = OpenSimplex::new(world_data.seed);
    let noise_continental = OpenSimplex::new(world_data.seed + 1);
    let noise_temperature = OpenSimplex::new(world_data.seed + 2);
    let noise_moisture = OpenSimplex::new(world_data.seed + 3);

    let scale_terrain = world_data.terrain_scale; //.005
    let scale_continental = world_data.continental_scale; //.0005
    let scale_temperature = world_data.temperature_scale;
    let scale_moisture = world_data.moisture_scale;

    let max_elevation = 100.0;
    let num_of_octaves = world_data.num_of_octaves;

    let mut squares: Vec<Square> = (0..WORLD_SIZE * WORLD_SIZE)
        .into_par_iter()
        .map(|i: i32| {
            let noise_terrain = noise_terrain.clone();
            let noise_continental = noise_continental.clone();

            let x = (i % WORLD_SIZE) as f64 / WORLD_SIZE as f64 * 2.0 * PI;
            let y = (i / WORLD_SIZE) as f64 / WORLD_SIZE as f64 * 2.0 * PI;

            let nx = x.cos() * world_data.scaling_factor;
            let ny = x.sin() * world_data.scaling_factor;
            let nz = y.cos() * world_data.scaling_factor;
            let nw = y.sin() * world_data.scaling_factor;

            let mut scale_terrain = scale_terrain;
            let mut amplitude = 1.0;
            let mut elevation_terrain = 0.0;
            let mut max_possible_amplitude = 0.0;

            for _i in 0..num_of_octaves {
                elevation_terrain += noise_terrain.get([
                    nx * scale_terrain,
                    ny * scale_terrain,
                    nz * scale_terrain,
                    nw * scale_terrain,
                ]) * amplitude;
                max_possible_amplitude += amplitude;

                scale_terrain = scale_terrain * 2.0;
                amplitude = amplitude / 2.0;
            }

            let elevation_continental = noise_continental.get([
                nx * scale_continental,
                ny * scale_continental,
                nz * scale_continental,
                nw * scale_continental,
            ]);

            let sea_bias = 0.075;

            let elevation_normalized = (elevation_continental - sea_bias)
                + ((elevation_terrain / max_possible_amplitude)
                    * get_land_strength(elevation_continental));

            let elevation_final = ((elevation_normalized + 1.0) / 2.0) * max_elevation;

            let min_temperature = -10.0;
            let max_temperature = 30.0;

            let y_lat = (i / WORLD_SIZE) as f64;

            let latitude = (y_lat - WORLD_SIZE as f64 / 2.0).abs() / (WORLD_SIZE as f64 / 2.0);

            let temperature_latitude = 30.0 - 40.0 * latitude;

            let h = elevation_final / max_elevation;
            let temperature_elevation = -h.powf(1.5) * 15.0;

            let temperature_noise_amplitude = 5.0;

            let temperature_noise = noise_temperature.get([
                nx * scale_temperature,
                ny * scale_temperature,
                nz * scale_temperature,
                nw * scale_temperature,
            ]) * temperature_noise_amplitude;

            let temperature_final =
                temperature_latitude + temperature_elevation + temperature_noise;

            let moisture_noise = noise_moisture.get([
                nx * scale_moisture,
                ny * scale_moisture,
                nz * scale_moisture,
                nw * scale_moisture,
            ]);

            let moisture_base = (moisture_noise + 1.0) / 2.0;
            let latitude = (y - WORLD_SIZE as f64 / 2.0).abs() / (WORLD_SIZE as f64 / 2.0);

            let equator_wet = (-latitude * 3.0).exp();
            let subtropical_dry = (-((latitude - 0.3).powi(2)) / 0.02).exp();

            let moisture_latitude = equator_wet - 0.4 * subtropical_dry;
            let moisture_elevation = -(elevation_final / max_elevation) * 0.25;

            let moisture_final =
                (moisture_base + moisture_latitude + moisture_elevation).clamp(0.0, 1.0);

            Square {
                elevation: elevation_final as f32,
                biome: Biome::Ocean, // Temporary, will be set later
                temperature: temperature_final as f32,
                moisture: moisture_final as f32,
            }
        })
        .collect();

    for i in 0..WORLD_SIZE * WORLD_SIZE {
        let rain_loss = 0.4;
        let upwind_i = if i == WORLD_SIZE * WORLD_SIZE - 1 {
            0
        } else {
            i + 1
        };

        let cur_elevation = squares[i as usize].elevation;
        let upwind_elevation = squares[(upwind_i) as usize].elevation;
        let upwind_moisture = squares[(upwind_i) as usize].moisture;
        let cur_temp = squares[i as usize].temperature;

        let mut moisture = upwind_moisture;

        let height_diff = (cur_elevation - upwind_elevation) / max_elevation as f32;

        if height_diff > 0.0 {
            moisture -= height_diff * rain_loss;
        }

        moisture = moisture.clamp(0.0, 1.0);

        squares[i as usize].moisture = moisture;
        squares[i as usize].biome = biome_from_climate(
            cur_temp as f64,
            moisture as f64,
            cur_elevation as f64,
            max_elevation,
        );
    }

    let world_map = WorldMap {
        width: WORLD_SIZE as u32,
        height: WORLD_SIZE as u32,
        squares: squares,
    };
    world_map
}

fn biome_from_climate(temp_c: f64, moisture: f64, elevation: f64, max_elevation: f64) -> Biome {
    let sea_level_elevation = max_elevation * SEA_LEVEL;

    if elevation < sea_level_elevation {
        return Biome::Ocean;
    }

    if temp_c < -10.0 {
        return Biome::Ice;
    }

    if elevation > 0.75 * max_elevation && temp_c <= 0.0 {
        return Biome::Snow;
    }

    if elevation > 0.6 * max_elevation && temp_c <= 2.0 {
        return Biome::Alpine;
    }

    match temp_c {
        t if t < -5.0 => {
            if moisture < 0.4 {
                Biome::Tundra
            } else {
                Biome::BorealForest
            }
        }

        t if t < 5.0 => {
            if moisture < 0.3 {
                Biome::Tundra
            } else {
                Biome::Taiga
            }
        }

        t if t < 18.0 => {
            if moisture < 0.2 {
                Biome::ColdDesert
            } else if moisture < 0.5 {
                Biome::Grassland
            } else if moisture < 0.75 {
                Biome::TemperateForest
            } else {
                Biome::TemperateRainforest
            }
        }

        t if t < 25.0 => {
            if moisture < 0.2 {
                Biome::HotDesert
            } else if moisture < 0.5 {
                Biome::Savanna
            } else {
                Biome::SubtropicalForest
            }
        }

        _ => {
            if moisture < 0.2 {
                Biome::HotDesert
            } else if moisture < 0.45 {
                Biome::Savanna
            } else {
                Biome::TropicalRainforest
            }
        }
    }
}

fn get_land_strength(elevation: f64) -> f64 {
    match elevation {
        -1.0 => 0.0,
        -1.0..=-0.5 => 0.1,
        -0.5..=0.0 => 0.5,
        0.0..=0.5 => 0.8,
        0.5..=1.0 => 1.0,
        _ => 0.0,
    }
}
