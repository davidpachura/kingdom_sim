use std::f64::consts::PI;

use crate::{
    components::{
        game_config::{
            ContinentalScaleField, InputValue, MoistureScaleField, OctaveField,
            ScalingFactorField, SeaThresholdField, SeedField, TerrainScaleField,
            TemperatureScaleField,
        },
        world::*,
        world_gen::WorldData,
    },
    states::game_state::*,
    systems::{game_config::*, main_menu::*},
};
use bevy::{
    asset::RenderAssetUsages, camera::Viewport, math::ops::powf, prelude::*,
    render::render_resource::PrimitiveTopology::TriangleList, window::WindowResolution,
};
use bevy_mesh::Indices;
use noise::{NoiseFn, OpenSimplex};
use rand::RngCore;
use rayon::prelude::*;
mod components;
mod states;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1600, 900),
                title: "Kingdom Sim".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            main_menu_buttons.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(GameState::WorldGenSetup), setup_game_config)
        .add_systems(
            Update,
            (
                game_config_buttons,
                game_config_text_input,
                update_text_display,
                focus_text_inputs,
            )
                .run_if(in_state(GameState::WorldGenSetup)),
        )
        .add_systems(
            OnExit(GameState::WorldGenSetup),
            (read_worldgen_inputs, cleanup_game_config).chain(),
        )
        .add_systems(OnEnter(GameState::WorldGenerating), generate_world)
        .add_systems(OnEnter(GameState::Playing), render_world)
        .add_systems(FixedUpdate, controls.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_world)
        .add_systems(Startup, setup)
        .run();
}

const WORLD_SIZE: i32 = 4096;
const CHUNK_SIZE: i32 = 256;
const CHUNKS_SIZE: i32 = WORLD_SIZE / CHUNK_SIZE;
const SEA_LEVEL: f64 = 0.48;

fn setup(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: window_size.as_uvec2(),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}

fn read_worldgen_inputs(
    mut commands: Commands,
    seed_query: Query<&InputValue, With<SeedField>>,
    terrain_scale_query: Query<&InputValue, With<TerrainScaleField>>,
    continental_scale_query: Query<&InputValue, With<ContinentalScaleField>>,
    octave_query: Query<&InputValue, With<OctaveField>>,
    sea_threshold_query: Query<&InputValue, With<SeaThresholdField>>,
    temperature_scale_query: Query<&InputValue, With<TemperatureScaleField>>,
    moisture_scale_query: Query<&InputValue, With<MoistureScaleField>>,
    scaling_factor_query: Query<&InputValue, With<ScalingFactorField>>,
) {
    let mut rng = rand::rng();
    let mut seed = rng.next_u32();
    let mut terrain_scale = 0.005;
    let mut continental_scale = 0.0005;
    let mut num_of_octaves = 4;
    let mut sea_threshold = 0.48;
    let mut temperature_scale = 0.005;
    let mut moisture_scale = 0.008;
    let mut scaling_factor = 100.0;

    for input in &seed_query {
        seed = input.text.parse::<u32>().unwrap_or(seed);
    }

    for input in &terrain_scale_query {
        terrain_scale = input.text.parse::<f64>().unwrap_or(0.005);
    }

    for input in &continental_scale_query {
        continental_scale = input.text.parse::<f64>().unwrap_or(0.000999);
    }

    for input in &octave_query {
        num_of_octaves = input.text.parse::<u32>().unwrap_or(20);
    }

    for input in &sea_threshold_query {
        sea_threshold = input.text.parse::<f64>().unwrap_or(0.48);
    }

    for input in &temperature_scale_query {
        temperature_scale = input.text.parse::<f64>().unwrap_or(0.0009);
    }

    for input in &moisture_scale_query {
        moisture_scale = input.text.parse::<f64>().unwrap_or(0.00099);
    }

    for input in &scaling_factor_query {
        scaling_factor = input.text.parse::<f64>().unwrap_or(1000.0);
    }

    commands.spawn(WorldData {
        seed: seed,
        terrain_scale: terrain_scale,
        continental_scale: continental_scale,
        num_of_octaves: num_of_octaves,
        sea_threshold: sea_threshold,
        temperature_scale: temperature_scale,
        moisture_scale: moisture_scale,
        scaling_factor: scaling_factor,
    });
}

fn generate_world(
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

fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&WorldMap>,
) {
    let world_map = match query.single() {
        Ok(map) => map,
        Err(err) => {
            error!("WorldMap query failed: {:?}", err);
            return;
        }
    };

    for chunk_x in 0..CHUNKS_SIZE {
        for chunk_y in 0..CHUNKS_SIZE {
            let mesh = generate_chunk(chunk_x, chunk_y, &world_map);

            commands.spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
                Transform::default(),
            ));
        }
    }
}

fn cleanup_world(
    mut commands: Commands,
    world_query: Query<Entity, With<WorldMap>>,
    world_data_query: Query<Entity, With<WorldData>>,
    mesh_query: Query<Entity, With<Mesh2d>>,
) {
    for entity in world_query {
        commands.entity(entity).despawn();
    }

    for entity in mesh_query {
        commands.entity(entity).despawn();
    }

    for entity in world_data_query {
        commands.entity(entity).despawn();
    }
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
        // println!(
        //     "Square {}: Biome {:?}, Elevation {}, Temp {}, Moisture {}",
        //     i,
        //     squares[i as usize].biome,
        //     squares[i as usize].elevation,
        //     squares[i as usize].temperature,
        //     squares[i as usize].moisture
        // );
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

fn biome_to_color(biome: Biome) -> [f32; 4] {
    match biome {
        Biome::Ocean => [0.0, 0.0, 0.5, 1.0],
        Biome::Coast => [0.8, 0.8, 0.3, 1.0],
        Biome::Grassland => [0.2, 0.8, 0.2, 1.0],
        Biome::Forest => [0.1, 0.5, 0.1, 1.0],
        Biome::Desert => [0.9, 0.8, 0.3, 1.0],
        Biome::Hill => [0.6, 0.5, 0.3, 1.0],
        Biome::Mountain => [0.5, 0.5, 0.5, 1.0],
        Biome::Ice => [0.68, 0.85, 0.90, 1.0],
        Biome::Alpine => [0.7, 0.7, 0.7, 1.0],
        Biome::Snow => [0.95, 0.95, 1.0, 1.0],
        Biome::Tundra => [0.8, 0.7, 0.6, 1.0],
        Biome::BorealForest => [0.2, 0.4, 0.2, 1.0],
        Biome::Taiga => [0.3, 0.5, 0.3, 1.0],
        Biome::ColdDesert => [0.8, 0.7, 0.5, 1.0],
        Biome::TemperateForest => [0.15, 0.6, 0.15, 1.0],
        Biome::TemperateRainforest => [0.1, 0.7, 0.2, 1.0],
        Biome::HotDesert => [1.0, 0.85, 0.3, 1.0],
        Biome::Savanna => [0.8, 0.8, 0.2, 1.0],
        Biome::SubtropicalForest => [0.2, 0.7, 0.3, 1.0],
        Biome::TropicalRainforest => [0.0, 0.6, 0.1, 1.0],
    }
}

fn generate_chunk(chunk_x: i32, chunk_y: i32, world_map: &WorldMap) -> Mesh {
    let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::default());
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let x_i32 = x + (chunk_x * CHUNK_SIZE);
            let y_i32 = y + (chunk_y * CHUNK_SIZE);

            let x = x_i32 as f32;
            let y = y_i32 as f32;

            let index = index_toroidal(x_i32, y_i32, WORLD_SIZE as i32);
            let square = &world_map.squares[index];

            positions.push([x, y, 0.0]); // v0
            positions.push([x + 1.0, y, 0.0]); // v1
            positions.push([x + 1.0, y + 1.0, 0.0]); // v2
            positions.push([x, y + 1.0, 0.0]); // v3

            let color = biome_to_color(square.biome);
            colors.push(color);
            colors.push(color);
            colors.push(color);
            colors.push(color);

            indices.extend_from_slice(&[
                index_offset,
                index_offset + 1,
                index_offset + 2,
                index_offset + 2,
                index_offset + 3,
                index_offset,
            ]);

            index_offset += 4;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    return mesh;
}

fn wrap(v: i32, max: i32) -> i32 {
    ((v % max) + max) % max
}

fn index_toroidal(x: i32, y: i32, size: i32) -> usize {
    let wx = wrap(x, size);
    let wy = wrap(y, size);
    (wy * size + wx) as usize
}

fn controls(
    camera_query: Single<(&mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time<Fixed>>,
) {
    let (mut transform, mut projection) = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();

    // Camera movement controls
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += fspeed;
    }

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }

    if input.pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
