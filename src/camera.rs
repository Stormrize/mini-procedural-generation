//! Ортографическая камера для чанкового мира.
//!
//! Управление:
//!   W/S или ↑/↓  — вперёд/назад (по изометрической оси)
//!   A/D или ←/→  — влево/вправо (по изометрической оси)
//!   Q / E        — zoom out / zoom in

use bevy::{camera::ScalingMode, prelude::*};

use crate::world::world_config::WorldConfig;

const MIN_ORTHOGRAPHIC_SCALE: f32 = 0.1;
const MAX_ORTHOGRAPHIC_SCALE: f32 = 5.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement);
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
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    // Фоновый свет — чтобы теневые грани не были чёрными.
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
        ..default()
    });

    // Направленный свет — даёт ощущение объёма тайлам.
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: false,
            ..default()
        },
        // Свет сверху-слева-спереди.
        Transform::from_xyz(-2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ─────────────────────────────────────────────────────────────────────────────

fn camera_movement(
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    // Скорость панорамирования.
    let pan_speed = 80.0 * time.delta_secs();

    // Изометрические оси движения в мировом пространстве.
    // "Вперёд" в изометрии — это одновременно -X и -Z.
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

    // Zoom через масштаб ортографической проекции.
    let zoom_speed = 0.04;
    let ground_anchor = camera_ground_point(&transform);
    if let Projection::Orthographic(ref mut ortho) = *projection {
        let previous_scale = ortho.scale;

        if keyboard.pressed(KeyCode::KeyE) {
            ortho.scale = (ortho.scale - zoom_speed).max(MIN_ORTHOGRAPHIC_SCALE);
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            ortho.scale = (ortho.scale + zoom_speed).min(MAX_ORTHOGRAPHIC_SCALE);
        }

        if ortho.scale > previous_scale {
            keep_orthographic_view_above_ground(&mut transform, ortho, ground_anchor);
        }
    }
}

fn camera_ground_point(transform: &Transform) -> Option<Vec3> {
    let forward = camera_forward(transform);
    if forward.y.abs() < f32::EPSILON {
        return None;
    }

    let distance = -transform.translation.y / forward.y;
    (distance > 0.0).then_some(transform.translation + forward * distance)
}

fn camera_forward(transform: &Transform) -> Vec3 {
    transform.rotation * Vec3::NEG_Z
}

fn keep_orthographic_view_above_ground(
    transform: &mut Transform,
    ortho: &OrthographicProjection,
    ground_anchor: Option<Vec3>,
) {
    let Some(ground_anchor) = ground_anchor else {
        return;
    };
    let Some(view_height) = orthographic_view_height(ortho) else {
        return;
    };

    let forward = camera_forward(transform);
    let up = transform.rotation * Vec3::Y;
    let downward = -forward.y;
    if downward <= f32::EPSILON {
        return;
    }

    let half_view_height = view_height * ortho.scale * 0.5;
    let ground_clearance = 2.0;
    let required_distance = (half_view_height * up.y.abs() + ground_clearance) / downward;
    let current_distance = (ground_anchor - transform.translation).dot(forward);

    if required_distance > current_distance {
        transform.translation = ground_anchor - forward * required_distance;
    }
}

fn orthographic_view_height(ortho: &OrthographicProjection) -> Option<f32> {
    match ortho.scaling_mode {
        ScalingMode::Fixed { height, .. } => Some(height),
        ScalingMode::FixedVertical { viewport_height } => Some(viewport_height),
        ScalingMode::AutoMin { min_height, .. } => Some(min_height),
        ScalingMode::AutoMax { max_height, .. } => Some(max_height),
        ScalingMode::WindowSize | ScalingMode::FixedHorizontal { .. } => None,
    }
}
