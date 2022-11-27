use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
}

// Screen width and height
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::hsl(0.0, 0.0, 0.2)))
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
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        // Our Systems
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_basic_camera)
        .add_system(tower_shooting_system)
        .add_system(bullet_despawn_system)
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a plane for the ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            // Green ground
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    // Spawn the cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            // Blue cube
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert(Name::new("Tower"));

    // Spawn a point light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_basic_camera(mut commands: Commands) {
    // Spawn the 3d camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// System for tower shooting
fn tower_shooting_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            // spawn a bullet
            let spawn_transfrom =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                    transform: spawn_transfrom,
                    ..default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

/// A lifetime component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

/// A bullet despawn system
fn bullet_despawn_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
