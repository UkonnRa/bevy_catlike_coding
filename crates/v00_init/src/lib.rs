use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_4;

pub struct TutorialPlugin {
    pub title: String,
    pub camera_transform: Transform,
}

impl Default for TutorialPlugin {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            camera_transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

impl TutorialPlugin {
    pub fn startup(mut commands: Commands, camera_transform: Transform) {
        commands.spawn((Camera3d::default(), camera_transform));

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
                maximum_distance: 10_000.0,
                ..default()
            }
            .build(),
        ));
    }

    pub fn camera_movement(
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
}

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        let camera_transform = self.camera_transform;
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: self.title.clone(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, move |commands: Commands| {
            TutorialPlugin::startup(commands, camera_transform)
        })
        .add_systems(Update, TutorialPlugin::camera_movement);
    }
}
