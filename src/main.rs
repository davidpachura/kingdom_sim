use bevy::{
    asset::RenderAssetUsages,
    camera::Viewport,
    color::palettes::{
        css::{GREEN},
    },
    math::ops::powf,
    prelude::*,
    render::render_resource::PrimitiveTopology::TriangleList,
    window::WindowResolution,
};
use bevy_mesh::Indices;

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
                physical_position: (window_size * 0.125).as_uvec2(),
                physical_size: (window_size * 0.75).as_uvec2(),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    let mut mesh = Mesh::new(TriangleList, RenderAssetUsages::default());

    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    let world_width = 256;
    let world_height = 256;

    for x in 0..world_width {
        for y in 0..world_height {
            let x = x as f32;
            let y = y as f32;

            // vertices
            positions.push([x,     y,     0.0]); // v0
            positions.push([x + 1.0, y,     0.0]); // v1
            positions.push([x + 1.0, y + 1.0, 0.0]); // v2
            positions.push([x,     y + 1.0, 0.0]); // v3

            // triangles
            indices.extend_from_slice(&[
                index_offset,     index_offset + 1, index_offset + 2,
                index_offset + 2, index_offset + 3, index_offset,
            ]);

            index_offset += 4;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::default(),
    ));
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (mut camera, mut transform, mut projection) = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();
    let uspeed = fspeed as u32;
    let window_size = window.resolution.physical_size();

    // Camera movement controls
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::ArrowRight) {
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

    if let Some(viewport) = camera.viewport.as_mut() {
        // Viewport movement controls
        if input.pressed(KeyCode::KeyW) {
            viewport.physical_position.y += uspeed;
        }
        if input.pressed(KeyCode::KeyS) {
            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyA) {
            viewport.physical_position.x += uspeed;
        }
        if input.pressed(KeyCode::KeyD) {
            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
        }

        // Bound viewport position so it doesn't go off-screen
        viewport.physical_position = viewport
            .physical_position
            .min(window_size - viewport.physical_size);
    }
}
