use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::prelude::shape::{Circle, Cylinder};
use bevy::prelude::Visibility::*;
use crate::cone::Cone;

#[derive(Component, Clone, PartialEq)]
pub struct Rocket3DObject;

pub fn emissive_material(color: Color, emissive: Color) -> StandardMaterial  {
    StandardMaterial{
        base_color: color,
        emissive,
        perceptual_roughness: 1.0,
        ..default()
    }
}

pub fn material(color: Color) -> StandardMaterial  {
    StandardMaterial{
        base_color: color,
        perceptual_roughness: 1.0,
        ..default()
    }
}
pub fn spawn_all_entities(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    standard_material: &mut ResMut<Assets<StandardMaterial>>,
) {
    let arrow_body = meshes.add(Mesh::from(shape::Cylinder { radius: 0.02, height: 0.9, resolution: 16, segments: 1 }));
    let arrow_head = meshes.add(Mesh::from(Cone { radius: 0.04, height: 0.1, resolution: 16 }));

    let red = standard_material.add(material(Color::RED));
    let green = standard_material.add(material(Color::GREEN));
    let blue = standard_material.add(material(Color::BLUE));
    let gray = standard_material.add(material(Color::GRAY));
    let silver = standard_material.add(material(Color::SILVER));
    let white = standard_material.add(material(Color::WHITE));
    let rocket_white = standard_material.add(emissive_material(Color::WHITE, Color::GRAY));
    let full_white = standard_material.add(emissive_material(Color::WHITE, Color::WHITE));
    let rocket_yellow = standard_material.add(emissive_material(Color::YELLOW, Color::OLIVE));


    let a = standard_material.add(emissive_material(Color::hex("#D6E650").unwrap(), Color::hex("#6B7328").unwrap()));
    let b = standard_material.add(emissive_material(Color::hex("#E65A50").unwrap(), Color::hex("#732D28").unwrap()));
    let c = standard_material.add(emissive_material(Color::hex("#519BE6").unwrap(), Color::hex("#284D73").unwrap()));

    macro_rules! rocket {
        ($transform: expr) => {
            let mut ec = commands.spawn((SpatialBundle {
                transform: $transform,
                visibility: Visible,
                ..default()
            }, Rocket3DObject));
            ec.with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cone {
                        height: 0.1,
                        radius: 0.05,
                        resolution: 16,
                    })),
                    material: rocket_white.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.4),
                    ..default()
                });
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cylinder {
                        height: 0.025,
                        radius: 0.05,
                        ..default()
                    })),
                    material: rocket_yellow.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_x(PI/2.0))
                        .with_translation(Vec3::new(0.0, 0.0, 0.3875)),
                    ..default()
                });
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cylinder {
                        height: 0.025,
                        radius: 0.05,
                        ..default()
                    })),
                    material: gray.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_x(PI/2.0))
                        .with_translation(Vec3::new(0.0, 0.0, 0.3625)),
                    ..default()
                });
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cylinder {
                        height: 0.75,
                        radius: 0.05,
                        ..default()
                    })),
                    material: rocket_white.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_x(PI/2.0))
                        .with_translation(Vec3::new(0.0, 0.0, -0.025)),
                    ..default()
                });
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: -0.15, max_x: -0.05,
                        min_y: -0.01, max_y: 0.01,
                        min_z: -0.5, max_z: -0.3 ,
                    })),
                    material: a.clone(),
                    ..default()
                });
                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: 0.0, max_x: 0.075,
                        min_y: -0.01, max_y: 0.01,
                        min_z: 0.0, max_z: 0.2 ,
                    })),
                    material: a.clone(),
                    transform: Transform::from_xyz(-0.15, 0.0, -0.3)
                        .with_rotation(Quat::from_rotation_y(0.7)),
                    ..default()
                });

                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: -0.15, max_x: -0.05,
                        min_y: -0.01, max_y: 0.01,
                        min_z: -0.5, max_z: -0.3 ,
                    })),
                    transform: Transform::from_rotation(Quat::from_rotation_z(2.0 * PI / 3.0)),
                    material: b.clone(),
                    ..default()
                });

                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: -0.0, max_x: 0.075,
                        min_y: -0.01, max_y: 0.01,
                        min_z: 0.0, max_z: 0.2 ,
                    })),
                    material: b.clone(),
                    transform: { let mut t = Transform::from_xyz(-0.15, 0.0, -0.3)
                        .with_rotation(Quat::from_rotation_y(0.7));
                        t.rotate_around(Vec3::ZERO, Quat::from_rotation_z(2.0 * PI / 3.0));
                        t},
                    ..default()
                });

                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: -0.15, max_x: -0.05,
                        min_y: -0.01, max_y: 0.01,
                        min_z: -0.5, max_z: -0.3 ,
                    })),
                    transform: Transform::from_rotation(Quat::from_rotation_z(-2.0 * PI / 3.0)),
                    material: c.clone(),
                    ..default()
                });

                p.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                       min_x: -0.0, max_x: 0.075,
                        min_y: -0.01, max_y: 0.01,
                        min_z: 0.0, max_z: 0.2 ,
                    })),
                    material: c.clone(),
                    transform: { let mut t = Transform::from_xyz(-0.15, 0.0, -0.3)
                        .with_rotation(Quat::from_rotation_y(0.7));
                        t.rotate_around(Vec3::ZERO, Quat::from_rotation_z(-2.0 * PI / 3.0));
                        t},
                    ..default()
                });

            });
            ec.id()
        };
    }


    rocket!(Transform::from_xyz(0.0, 0.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Circle::new(10.0))),
        material: gray.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -0.5),
        ..default()
    });
}