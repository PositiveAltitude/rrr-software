use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use crate::visual_objects::{Rocket3DObject, spawn_all_entities};
use std::thread;
use std::sync::mpsc::channel;


mod simulation;
mod visual_objects;
mod cone;

fn main() {
    let (tx, rx) = channel::<f32>();

    App::new()
        .insert_resource(Msaa::Sample8)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1800.0,
            radius: 0.1,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(6.0, 2.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1100.0,
            radius: 0.1,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-6.0, 1.0, 10.0),
        ..default()
    });


    commands.spawn((Camera3dBundle {
        camera: Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(3.0, -9.0, 5.0)              //(16.0, -32.0, 15.5)
            .looking_at(Vec3::new(0.0, 0.0, 3.0), Vec3::Z),
        ..default()
    }, BloomSettings{
        intensity: 0.15,
        // low_frequency_boost: 1.0,
        // low_frequency_boost_curvature:1.0,
        //composite_mode: Additive,
        ..default()
    }));

    spawn_all_entities(
        &mut commands, &mut meshes, &mut materials,
    );
}

pub fn update(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Rocket3DObject>)>,
    mut rocket_3d_objects: Query<(&mut Transform, &mut Visibility, &Rocket3DObject), Without<Camera>>,
    mut gizmos: Gizmos,
) {

}