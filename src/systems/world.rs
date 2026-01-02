use bevy::{
    asset::RenderAssetUsages, math::ops::powf, prelude::*,
    render::render_resource::PrimitiveTopology::TriangleList,
};
use bevy_mesh::Indices;

use crate::components::world::*;
use crate::states::game_state::GameState;

const WORLD_SIZE: i32 = 4096;
const CHUNK_SIZE: i32 = 256;
const CHUNKS_SIZE: i32 = WORLD_SIZE / CHUNK_SIZE;

pub fn render_world(
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

pub fn cleanup_world(
    mut commands: Commands,
    world_query: Query<Entity, With<WorldMap>>,
    world_data_query: Query<Entity, With<crate::components::world_gen::WorldData>>,
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

pub fn controls(
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
