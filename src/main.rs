use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, orbit_camera_controls_system)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let zoom = 20.;
    commands.spawn((Camera3dBundle {
        camera: Camera { ..default() },
        transform: Transform::from_xyz(-2.0 * zoom, 2.5 * zoom, 5.0 * zoom)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));

    let material = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    let mesh = meshes.add(shape::Cube { size: 1. }.try_into().unwrap());

    for x in -20..20 {
        for y in -20..20 {
            for z in -20..20 {
                let mut hasher = DefaultHasher::new();
                (x, y, z).hash(&mut hasher);
                let rand = (hasher.finish() - 2) % 3;
                if rand > 0 {
                    continue;
                }

                commands.spawn((PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },));
            }
        }
    }
}

fn orbit_camera_controls_system(
    mut camera_query: Query<(&mut Camera, &mut Transform)>,
    timer: Res<Time>,
) {
    let (_, mut transform) = camera_query.single_mut();

    // Nope nope nope, this rotates the caemra itself 😂
    // transform.rotate_y(0.3 * TAU * timer.delta_seconds())

    // I can't make this work wtf
    // transform.rotate_around(
    //     Vec3::from((0., 0., 0.)),
    //     Quat::from_array([0., 0., 0., 0.1]),
    // )

    transform.translation.x = (timer.elapsed_seconds() * 5.).sin() * 20.;
    transform.translation.y = (timer.elapsed_seconds() * 5.).cos() * 20.;
    transform.look_at(Vec3::from((0., 0., 0.)), Vec3::Y);
}
