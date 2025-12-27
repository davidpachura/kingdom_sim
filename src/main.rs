use bevy::{
    asset::RenderAssetUsages,
    camera::Viewport,
    math::ops::powf,
    prelude::*,
    render::render_resource::PrimitiveTopology::TriangleList,
    window::WindowResolution,
};
use bevy_mesh::Indices;
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;

use crate::components::world_map::*;
mod components;

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
    .add_systems(Startup, setup)
    .add_systems(FixedUpdate, controls)
    .run();
}

const WORLD_SIZE: i32 = 4096;
const CHUNK_SIZE: i32 = 256;
const CHUNKS_SIZE: i32 = WORLD_SIZE/CHUNK_SIZE;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>
) {
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

    let world_map = generate_logical_world();
    
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

fn generate_logical_world() -> WorldMap {
    let noise = OpenSimplex::new(12345);
    let scale = 0.01;
    let max_elevation = 100;

    let mut squares: Vec<Square> = (0..WORLD_SIZE * WORLD_SIZE)
    .into_par_iter()
    .map(|i| {
        let noise = noise.clone();

        let x = i % WORLD_SIZE;
        let y = i / WORLD_SIZE;

        let elevation = noise.get([x as f64 * scale, y as f64 * scale]);
        let elevation_f32 = ((elevation + 1.0)/2.0 * max_elevation as f64) as f32;

        let biome = if elevation_f32 <= 30.0 {
            Biome::Ocean
        } else if elevation_f32 <= 70.0 {
            Biome::Grassland
        } else {
            Biome::Mountain
        };

        Square { elevation: elevation_f32, biome }
    })
    .collect();

    let world_map = WorldMap {width: WORLD_SIZE as u32, height: WORLD_SIZE as u32, squares: squares};
    world_map
}

fn generate_chunk(
    chunk_x: i32,
    chunk_y: i32,
    world_map: &WorldMap,
) -> Mesh {
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

            let index = ((y_i32 * WORLD_SIZE) + x_i32) as usize;
            let square = &world_map.squares[index];

            // vertices
            positions.push([x,     y,     0.0]); // v0
            positions.push([x + 1.0, y,     0.0]); // v1
            positions.push([x + 1.0, y + 1.0, 0.0]); // v2
            positions.push([x,     y + 1.0, 0.0]); // v3

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

            // triangles
            indices.extend_from_slice(&[
                index_offset,     index_offset + 1, index_offset + 2,
                index_offset + 2, index_offset + 3, index_offset,
            ]);

            index_offset += 4;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    return mesh;
}

fn controls(
    camera_query: Single<(&mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
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
}
