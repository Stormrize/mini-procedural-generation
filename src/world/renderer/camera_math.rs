use bevy::{prelude::*, camera::ScalingMode};
use crate::world::math::math::min_max_chunk;
use crate::world::world_config::{WorldConfig, WorldShape, CHUNK_SIZE};

pub fn camera_forward(transform: &Transform) -> Vec3 {
    transform.rotation * Vec3::NEG_Z
}

pub fn project_ray_to_ground(origin: Vec3, direction: Vec3) -> Option<Vec3> {
    if direction.y.abs() < f32::EPSILON { return None; }
    let distance = -origin.y / direction.y;
    Some(origin + direction * distance)
}

pub fn camera_ground_point(transform: &Transform) -> Vec3 {
    project_ray_to_ground(transform.translation, camera_forward(transform))
        .unwrap_or(transform.translation)
}

pub fn orthographic_view_size(ortho: &OrthographicProjection, window: &Window) -> (f32, f32) {
    let aspect = (window.width() / window.height()).max(0.001);

    let (width, height) = match ortho.scaling_mode {
        ScalingMode::WindowSize => (window.width(), window.height()),
        ScalingMode::Fixed { width, height } => (width, height),
        ScalingMode::AutoMin {
            min_width,
            min_height,
        } => {
            if aspect > min_width / min_height {
                (min_height * aspect, min_height)
            } else {
                (min_width, min_width / aspect)
            }
        }
        ScalingMode::AutoMax {
            max_width,
            max_height,
        } => {
            if aspect < max_width / max_height {
                (max_height * aspect, max_height)
            } else {
                (max_width, max_width / aspect)
            }
        }
        ScalingMode::FixedVertical { viewport_height } => {
            (viewport_height * aspect, viewport_height)
        }
        ScalingMode::FixedHorizontal { viewport_width } => {
            (viewport_width, viewport_width / aspect)
        }
    };

    (width * ortho.scale, height * ortho.scale)
}

pub fn camera_chunk_bounds(
    transform: &Transform,
    projection: &Projection,
    window: &Window,
    config: &WorldConfig,
) -> (IVec2, IVec2) {
    let center = camera_ground_point(transform);
    let center_chunk = chunk_coord_from_world_xz(center.x, center.z, config);

    let mut min = center_chunk;
    let mut max = center_chunk;

    if let Projection::Orthographic(ortho) = projection {
        let (view_width, view_height) = orthographic_view_size(ortho, window);
        let right = transform.rotation * Vec3::X;
        let up = transform.rotation * Vec3::Y;
        let forward = camera_forward(transform);
        let origin = transform.translation;
        let half_right = right * (view_width * 0.5);
        let half_up = up * (view_height * 0.5);
        let corners = [
            origin - half_right - half_up,
            origin - half_right + half_up,
            origin + half_right - half_up,
            origin + half_right + half_up,
        ];

        for corner in corners {
            if let Some(point) = project_ray_to_ground(corner, forward) {
                let coord = chunk_coord_from_world_xz(point.x, point.z, config);
                min = min.min(coord);
                max = max.max(coord);
            }
        }
    }

    let preload = IVec2::splat(config.preload_margin_chunks as i32);
    min -= preload;
    max += preload;

    (min, max)
}

pub fn chunk_coord_from_world_xz(world_x: f32, world_z: f32, config: &WorldConfig) -> IVec2 {
    let chunk_size = config.chunk_world_size();
    IVec2::new(
        (world_x / chunk_size).floor() as i32,
        (world_z / chunk_size).floor() as i32,
    )
}

pub fn finite_world_chunk_bounds(config: &WorldConfig) -> Option<(IVec2, IVec2)> {
    let WorldShape::Island {
        center_tile,
        width_tiles,
        height_tiles,
        ..
    } = &config.world_shape
    else {
        return None;
    };
    let margin = config.preload_margin_chunks.max(0) as i32;
        
    Some(min_max_chunk(*width_tiles as i32, *height_tiles as i32, *center_tile, margin))
}

pub fn outside_bounds(coord: IVec2, min: IVec2, max: IVec2) -> bool {
    coord.x < min.x || coord.y < min.y || coord.x > max.x || coord.y > max.y
}
