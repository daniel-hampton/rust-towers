use bevy::prelude::*;

// Screen width and height
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_basic_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Moose Tower Defense".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a plane for the ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        // Green ground
        material: materials.add(Color::hsl(120.0, 0.5, 0.5).into()),
        ..default()
    });

    // Spawn the cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        // Blue cube
        material: materials.add(Color::hsl(220.0, 0.5, 0.75).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn spawn_basic_camera(mut commands: Commands) {
    // Spawn the 3d camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
