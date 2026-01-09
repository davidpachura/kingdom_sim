use crate::{
    components::{
        game_config::{
            ContinentalScaleField, InputValue, MoistureScaleField, OctaveField, ScalingFactorField,
            SeaThresholdField, SeedField, TemperatureScaleField, TerrainScaleField,
        },
        world::*,
        world_gen::WorldData,
    },
    states::game_state::*,
    systems::{game_config::*, main_menu::*, world::*, world_gen::generate_world},
};
use bevy::{
    camera::Viewport, platform::collections::HashMap, prelude::*, window::WindowResolution,
};
use rand::RngCore;
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
        .insert_resource(CameraChunk::default())
        .insert_resource(LoadedChunks {
            chunks: HashMap::new(),
        })
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
        // .add_systems(OnEnter(GameState::WorldGenerating), generate_world)
        // .add_systems(OnEnter(GameState::Playing), (render_world, setup_biome_display).chain())
        .add_systems(
            Update,
            (update_camera_chunk, update_chunks)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // .add_systems(Update, update_biome_display.run_if(in_state(GameState::Playing)))
        .add_systems(FixedUpdate, controls.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_world)
        .add_systems(Startup, setup)
        .run();
}

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

fn update_camera_chunk(
    camera_q: Query<&Transform, With<Camera>>,
    mut camera_chunk: ResMut<CameraChunk>,
) {
    let transform = camera_q.single();
    let chunk = world_pos_to_chunk(transform.unwrap().translation);
    camera_chunk.x = chunk.x;
    camera_chunk.y = chunk.y;
}

fn world_pos_to_chunk(pos: Vec3) -> IVec2 {
    IVec2::new(
        (pos.x.floor() as i32).div_euclid(CHUNK_SIZE),
        (pos.y.floor() as i32).div_euclid(CHUNK_SIZE),
    )
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
        temperature_scale = input.text.parse::<f64>().unwrap_or(0.0005);
    }

    for input in &moisture_scale_query {
        moisture_scale = input.text.parse::<f64>().unwrap_or(0.0008);
    }

    for input in &scaling_factor_query {
        scaling_factor = input.text.parse::<f64>().unwrap_or(1000.0);
    }

    println!("World data");
    println!("Seed: {0}", seed);
    println!("T_Scale {0}", terrain_scale);
    println!("C_Scale {0}", continental_scale);
    println!("Temp_Scale {0}", temperature_scale);
    println!("Moist_Scale {0}", moisture_scale);
    println!("O_num: {0}", num_of_octaves);
    println!("S_Threshold {0}", sea_threshold);
    println!("Scaling_Factor {0}", scaling_factor);

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
