use std::f64::consts::PI;

use crate::{
    components::{
        game_config::{
            ContinentalScaleField, InputValue, MountainThresholdField, OctaveField,
            ScalingFactorField, SeaThresholdField, SeedField, TerrainScaleField,
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
    mountain_threshold_query: Query<&InputValue, With<MountainThresholdField>>,
    scaling_factor_query: Query<&InputValue, With<ScalingFactorField>>,
) {
    let mut rng = rand::rng();
    let mut seed = rng.next_u32();
    let mut terrain_scale = 0.005;
    let mut continental_scale = 0.0005;
    let mut num_of_octaves = 4;
    let mut sea_threshold = 0.48;
    let mut mountain_threshold = 0.70;
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

    for input in &mountain_threshold_query {
        mountain_threshold = input.text.parse::<f64>().unwrap_or(0.70);
    }

    for input in &scaling_factor_query {
        scaling_factor = input.text.parse::<f64>().unwrap_or(100.0);
    }

    commands.spawn(WorldData {
        seed: seed,
        terrain_scale: terrain_scale,
        continental_scale: continental_scale,
        num_of_octaves: num_of_octaves,
        sea_threshold: sea_threshold,
        mountain_threshold: mountain_threshold,
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
    println!("O_num: {0}", world_data.num_of_octaves);
    println!("S_Threshold {0}", world_data.sea_threshold);
    println!("M_Threshold {0}", world_data.mountain_threshold);
    println!("Scaling_Factor {0}", world_data.scaling_factor);
    let noise_terrain = OpenSimplex::new(world_data.seed);
    let noise_continental = OpenSimplex::new(world_data.seed + 1);
    let scale_terrain = world_data.terrain_scale; //.005
    let scale_continental = world_data.continental_scale; //.0005
    let max_elevation = 100.0;
    let num_of_octaves = world_data.num_of_octaves;

    let squares: Vec<Square> = (0..WORLD_SIZE * WORLD_SIZE)
        .into_par_iter()
        .map(|i| {
            let noise_terrain = noise_terrain.clone();
            let noise_continental = noise_continental.clone();

            let x = (i % WORLD_SIZE) as f64 / WORLD_SIZE as f64 * 2.0 * PI;
            let y = (i / WORLD_SIZE) as f64 / WORLD_SIZE as f64 * 2.0 * PI;

            let mut scale_terrain = scale_terrain;
            let mut amplitude = 1.0;
            let mut elevation_terrain = 0.0;
            let mut max_possible_amplitude = 0.0;

            let scaling_factor = world_data.scaling_factor;

            for _i in 0..num_of_octaves {
                elevation_terrain += noise_terrain.get([
                    x.cos() * scaling_factor * scale_terrain,
                    x.sin() * scaling_factor * scale_terrain,
                    y.cos() * scaling_factor * scale_terrain,
                    y.sin() * scaling_factor * scale_terrain,
                ]) * amplitude;
                max_possible_amplitude += amplitude;

                scale_terrain = scale_terrain * 2.0;
                amplitude = amplitude / 2.0;
            }

            let elevation_continental = noise_continental.get([
                x.cos() * scaling_factor * scale_continental,
                x.sin() * scaling_factor * scale_continental,
                y.cos() * scaling_factor * scale_continental,
                y.sin() * scaling_factor * scale_continental,
            ]);

            let sea_bias = 0.075;

            let elevation_normalized = (elevation_continental - sea_bias)
                + ((elevation_terrain / max_possible_amplitude)
                    * get_land_strength(elevation_continental));

            let elevation_final = ((elevation_normalized + 1.0) / 2.0) * max_elevation;

            let biome = if elevation_final <= (max_elevation * world_data.sea_threshold) {
                Biome::Ocean
            } else if elevation_final <= (max_elevation * world_data.mountain_threshold) {
                Biome::Grassland
            } else {
                Biome::Mountain
            };

            Square {
                elevation: elevation_final as f32,
                biome,
            }
        })
        .collect();

    let world_map = WorldMap {
        width: WORLD_SIZE as u32,
        height: WORLD_SIZE as u32,
        squares: squares,
    };
    world_map
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

            if square.biome == Biome::Ocean {
                colors.push([0.0, 0.0, 1.0, 1.0]);
                colors.push([0.0, 0.0, 1.0, 1.0]);
                colors.push([0.0, 0.0, 1.0, 1.0]);
                colors.push([0.0, 0.0, 1.0, 1.0]);
            } else if square.biome == Biome::Grassland {
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
            } else {
                colors.push([0.5, 0.5, 0.5, 1.0]);
                colors.push([0.5, 0.5, 0.5, 1.0]);
                colors.push([0.5, 0.5, 0.5, 1.0]);
                colors.push([0.5, 0.5, 0.5, 1.0]);
            }

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
