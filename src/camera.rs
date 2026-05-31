//! Изометрическая ортографическая камера (45°, как Civilization/XCOM).
//!
//! Камера стоит вдоль диагонали (1, 1, 1) от центра карты, смотрит вниз —
//! это даёт классический вид «сверху-сбоку» без перспективных искажений.
//!
//! Управление:
//!   W/S или ↑/↓  — вперёд/назад (по изометрической оси)
//!   A/D или ←/→  — влево/вправо (по изометрической оси)
//!   Q / E        — zoom out / zoom in

use bevy::{camera::ScalingMode, prelude::*};

use crate::world::chunk::config::WorldConfig;

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
    // Центр карты в мировых координатах.
    let cx = config.map_width  as f32 * config.tile_size * 0.5;
    let cz = config.map_height as f32 * config.tile_size * 0.5;

    // Дистанция камеры зависит от размера карты.
    let map_size = config.map_width.max(config.map_height) as f32 * config.tile_size;
    let dist     = map_size * 0.85;

    // Истинная изометрия: смещение одинаково по X, Y, Z.
    let camera_pos = Vec3::new(cx + dist, dist, cz + dist);
    let look_at    = Vec3::new(cx, 0.0, cz);

    // viewport_height — сколько мировых единиц помещается по высоте экрана.
    // Карта 64×64 выглядит как ромб ~90 ед. диагональю → берём с запасом.
    let viewport_h = map_size * 1.4;

    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: viewport_h,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(look_at, Vec3::Y),
    ));

    // Фоновый свет — чтобы теневые грани не были чёрными.
    commands.insert_resource(GlobalAmbientLight {
        color:      Color::WHITE,
        brightness: 400.0,
        ..default()
    });

    // Направленный свет — даёт ощущение объёма тайлам.
    commands.spawn((
        DirectionalLight {
            illuminance:      8_000.0,
            shadows_enabled:  false,
            ..default()
        },
        // Свет сверху-слева-спереди.
        Transform::from_xyz(-2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ─────────────────────────────────────────────────────────────────────────────

fn camera_movement(
    mut camera_q: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    keyboard:     Res<ButtonInput<KeyCode>>,
    time:         Res<Time>,
) {
    let Ok((mut transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    // Скорость панорамирования.
    let pan_speed = 18.0 * time.delta_secs();

    // Изометрические оси движения в мировом пространстве.
    // "Вперёд" в изометрии — это одновременно -X и -Z.
    let iso_forward = Vec3::new(-1.0, 0.0, -1.0).normalize();
    let iso_right   = Vec3::new( 1.0, 0.0, -1.0).normalize();

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
    if let Projection::Orthographic(ref mut ortho) = *projection {
        if keyboard.pressed(KeyCode::KeyE) {
            ortho.scale = (ortho.scale - zoom_speed).max(0.1);
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            ortho.scale = (ortho.scale + zoom_speed).min(5.0);
        }
    }
}
