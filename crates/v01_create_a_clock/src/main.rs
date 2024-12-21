use bevy::color::palettes::css::{BLACK, GRAY, RED};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use chrono::Timelike;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

#[derive(Resource, Default, Debug)]
struct ClockData {
    pub real_time: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "V01: Create A Clock".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClockData { real_time: true })
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, clock_rotate))
        .run();
}

const DEFAULT_SECOND: Transform = Transform::from_xyz(0.0, 0.0, 4.0);
const DEFAULT_MINUTE: Transform = Transform::from_xyz(0.0, 0.0, 2.75);
const DEFAULT_HOUR: Transform = Transform::from_xyz(0.0, 0.0, 1.5);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 30.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-FRAC_PI_4),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

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

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    let speed = 10.0;

    let mut x_dir = 0.0;
    let mut y_dir = 0.0;
    let mut z_dir = 0.0;
    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyW => z_dir += 1.0,
            KeyCode::KeyS => z_dir += -1.0,
            KeyCode::KeyA => x_dir += -1.0,
            KeyCode::KeyD => x_dir += 1.0,
            KeyCode::KeyQ => y_dir += -1.0,
            KeyCode::KeyE => y_dir += 1.0,
            _ => {}
        }
    }

    for mut transform in camera_query.iter_mut() {
        let right = x_dir * transform.right();
        let up = y_dir * transform.up();
        let forward = z_dir * transform.forward();
        transform.translation += (right + up + forward) * speed * time.delta_secs();
    }
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
