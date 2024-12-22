use bevy::color::palettes::css::{BLACK, GRAY, RED};
use bevy::prelude::*;
use chrono::Timelike;
use std::f32::consts::{FRAC_PI_2, TAU};
use v00_init::TutorialPlugin;

#[derive(Resource, Default, Debug)]
struct ClockData {
    pub real_time: bool,
}

fn main() {
    App::new()
        .add_plugins(TutorialPlugin {
            title: "V01: Create A Clock".to_string(),
            ..Default::default()
        })
        .insert_resource(ClockData { real_time: true })
        .add_systems(Startup, startup)
        .add_systems(Update, clock_rotate)
        .run();
}

const DEFAULT_SECOND: Transform = Transform::from_xyz(0.0, 0.0, 4.0);
const DEFAULT_MINUTE: Transform = Transform::from_xyz(0.0, 0.0, 2.75);
const DEFAULT_HOUR: Transform = Transform::from_xyz(0.0, 0.0, 1.5);

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cylinder::new(10.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        ))
        .with_children(|child_builder| {
            for i in 0..12 {
                let mut transform = Transform::from_xyz(0.0, 0.0, 8.0);
                transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(i as f32 * TAU / 12.0));
                child_builder.spawn((
                    Name::new("Clock Indicator"),
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 3.0))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(GRAY))),
                    transform,
                ));
            }

            child_builder.spawn((
                Name::new("Hour Indicator"),
                Mesh3d(meshes.add(Cuboid::new(0.5, 1.5, 3.0))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(BLACK))),
                DEFAULT_HOUR,
            ));

            child_builder.spawn((
                Name::new("Minute Indicator"),
                Mesh3d(meshes.add(Cuboid::new(0.25, 1.5, 6.0))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(BLACK))),
                DEFAULT_MINUTE,
            ));
            child_builder.spawn((
                Name::new("Second Indicator"),
                Mesh3d(meshes.add(Cuboid::new(0.1, 1.5, 9.0))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
                DEFAULT_SECOND,
            ));
        });
}

fn clock_rotate(
    time: Res<Time>,
    clock_data: Res<ClockData>,
    mut camera_query: Query<(&mut Transform, &Name), With<Mesh3d>>,
) {
    for (mut transform, name) in camera_query.iter_mut() {
        let (rot_sec, rot_min, rot_hour) = if clock_data.real_time {
            let now = chrono::Local::now().time();
            (
                now.second() as f32 / 60.0 * TAU,
                now.minute() as f32 / 60.0 * TAU,
                now.hour() as f32 / 12.0 * TAU,
            )
        } else {
            (
                TAU * time.delta_secs() / 60.0,
                TAU * time.delta_secs() / 3600.0,
                TAU * time.delta_secs() / (3600.0 * 60.0),
            )
        };

        if name.as_str() == "Second Indicator" {
            if clock_data.real_time {
                *transform = DEFAULT_SECOND;
            }
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rot_sec));
        } else if name.as_str() == "Minute Indicator" {
            if clock_data.real_time {
                *transform = DEFAULT_MINUTE;
            }
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rot_min));
        } else if name.as_str() == "Hour Indicator" {
            if clock_data.real_time {
                *transform = DEFAULT_HOUR;
            }
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rot_hour));
        }
    }
}
