//! Ортографическая камера для чанкового мира.

use bevy::{camera::ScalingMode, prelude::*};

use crate::world::world_config::WorldConfig;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

const MIN_ORTHOGRAPHIC_SCALE: f32 = 0.1;
const MAX_ORTHOGRAPHIC_SCALE: f32 = 60.0;


#[derive(Resource)]
struct CameraZoom {
    zoom: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraZoom { zoom: 1.0 })
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_movement, apply_zoom_smoothing).chain());
    }
}

/// Маркер главной камеры — чтобы легко найти её в Query.
#[derive(Component)]
pub struct MainCamera;

// ─────────────────────────────────────────────────────────────────────────────

fn setup_camera(mut commands: Commands, config: Res<WorldConfig>) {
    let chunk_size = config.chunk_world_size();
    let look_at = Vec3::new(chunk_size * 0.5, 0.0, chunk_size * 0.5);
    let dist = chunk_size * 0.85;
    let camera_pos = look_at + Vec3::new(dist, dist, dist);

    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.camera_viewport_height,
            },
            near: -10_000.0,
            far: 10_000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_movement(
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut target_zoom: ResMut<CameraZoom>,
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    let pan_speed = 80.0 * time.delta_secs();

    let iso_forward = Vec3::new(-1.0, 0.0, -1.0).normalize();
    let iso_right = Vec3::new(1.0, 0.0, -1.0).normalize();

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation += iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation -= iso_forward * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation += iso_right * pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation -= iso_right * pan_speed;
    }

    let zoom_speed = 0.2;
    if let Projection::Orthographic(ref mut ortho) = *projection {

        for mouse_wheel in mouse_wheel_reader.read() {

            let scroll = match mouse_wheel.unit {
                MouseScrollUnit::Line => mouse_wheel.y * zoom_speed,
                MouseScrollUnit::Pixel => mouse_wheel.y * zoom_speed * 0.05,
            };

            target_zoom.zoom = (target_zoom.zoom - scroll).clamp(MIN_ORTHOGRAPHIC_SCALE, MAX_ORTHOGRAPHIC_SCALE);
        }
    }
}
fn apply_zoom_smoothing(
    mut camera_q: Query<&mut Projection, With<MainCamera>>,
    target_zoom: Res<CameraZoom>,
    time: Res<Time>,
) {
    let Ok(mut projection) = camera_q.single_mut() else {
        return;
    };

    let Projection::Orthographic(ref mut ortho) = *projection else {
        return;
    };

    let smoothness = 8.0;
    let t = 1.0 - (-smoothness * time.delta_secs()).exp();

    ortho.scale = ortho.scale.lerp(target_zoom.zoom, t);
}