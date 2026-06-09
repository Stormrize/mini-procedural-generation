//! WorldPlugin: конфиг, стриминг чанков и реакция на regenerate.

pub mod renderer;

use bevy::{camera::ScalingMode, prelude::*, window::PrimaryWindow};
use std::collections::HashMap;

use crate::camera::MainCamera;
use crate::world::renderer::renderer::{
    SpawnedChunkMeshes, TileAssets, despawn_chunk_meshes, spawn_chunk_meshes,
};
use crate::world::world::World;
use crate::world::world_config::WorldConfig;

#[derive(Resource, Default)]
struct RenderedChunks {
    chunks: HashMap<IVec2, SpawnedChunkMeshes>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default())
            .insert_resource(World::default())
            .insert_resource(RenderedChunks::default())
            .add_message::<RegenerateWorldEvent>()
            .add_systems(Startup, setup_tile_assets)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks).chain(),
            );
    }
}

#[derive(Message)]
pub struct RegenerateWorldEvent;

fn setup_tile_assets(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(TileAssets::build(&mut materials));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WorldConfig>,
    mut events: MessageWriter<RegenerateWorldEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        config.seed = config.seed.wrapping_add(1);
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        events.write(RegenerateWorldEvent);
        info!("Случайный seed: {}", config.seed);
    }
}

fn apply_regenerate(
    mut events: MessageReader<RegenerateWorldEvent>,
    mut commands: Commands,
    mut world: ResMut<World>,
    mut rendered: ResMut<RenderedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if events.read().count() == 0 {
        return;
    }

    for (_, chunk_meshes) in rendered.chunks.drain() {
        despawn_chunk_meshes(&mut commands, &mut meshes, chunk_meshes);
    }
    world.clear();
}

fn stream_chunks(
    mut commands: Commands,
    config: Res<WorldConfig>,
    mut world: ResMut<World>,
    mut rendered: ResMut<RenderedChunks>,
    tile_assets: Res<TileAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_q: Query<(&Transform, &Projection), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((camera_transform, projection)) = camera_q.single() else {
        return;
    };
    let Ok(window) = window_q.single() else {
        return;
    };

    let center = camera_ground_point(camera_transform);
    let center_chunk = chunk_coord_from_world_xz(center.x, center.z, &config);
    let (load_min, load_max) = camera_chunk_bounds(camera_transform, projection, window, &config);
    let unload_padding = IVec2::splat(config.unload_margin_chunks);
    let unload_min = load_min - unload_padding;
    let unload_max = load_max + unload_padding;

    let to_unload: Vec<IVec2> = rendered
        .chunks
        .keys()
        .copied()
        .filter(|coord| outside_bounds(*coord, unload_min, unload_max))
        .collect();

    for coord in to_unload {
        if let Some(chunk_meshes) = rendered.chunks.remove(&coord) {
            despawn_chunk_meshes(&mut commands, &mut meshes, chunk_meshes);
        }
        world.unload_chunk(coord);
    }

    let mut missing = Vec::new();
    for y in load_min.y..=load_max.y {
        for x in load_min.x..=load_max.x {
            let coord = IVec2::new(x, y);
            if rendered.chunks.contains_key(&coord) {
                continue;
            }
            missing.push(coord);
        }
    }

    missing.sort_by_key(|coord| {
        let delta = *coord - center_chunk;
        let chebyshev = delta.x.abs().max(delta.y.abs());
        let manhattan = delta.x.abs() + delta.y.abs();
        (chebyshev, manhattan)
    });

    let mut loaded = 0usize;
    for coord in missing.into_iter().take(config.max_chunk_loads_per_frame) {
        if rendered.chunks.contains_key(&coord) {
            continue;
        }

        let chunk = world.chunk_or_generate(coord, &config);
        let chunk_meshes =
            spawn_chunk_meshes(&mut commands, &mut meshes, chunk, &config, &tile_assets);
        rendered.chunks.insert(coord, chunk_meshes);
        loaded += 1;
    }

    if loaded > 0 {
        info!(
            "Loaded {} chunk(s). Center: {:?}, bounds: {:?}..{:?}, total loaded: {}",
            loaded,
            center_chunk,
            load_min,
            load_max,
            rendered.chunks.len()
        );
    }
}

fn camera_ground_point(transform: &Transform) -> Vec3 {
    let origin = transform.translation;
    let direction = camera_forward(transform);

    if direction.y.abs() < f32::EPSILON {
        return origin;
    }

    let distance = -origin.y / direction.y;
    origin + direction * distance
}

fn camera_chunk_bounds(
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

    let preload = IVec2::splat(config.preload_margin_chunks);
    min -= preload;
    max += preload;

    let min_radius = config.min_loaded_radius_chunks;
    min = min.min(center_chunk - IVec2::splat(min_radius));
    max = max.max(center_chunk + IVec2::splat(min_radius));

    let max_radius = config
        .max_loaded_radius_chunks
        .max(config.min_loaded_radius_chunks);
    min = min.max(center_chunk - IVec2::splat(max_radius));
    max = max.min(center_chunk + IVec2::splat(max_radius));

    (min, max)
}

fn camera_forward(transform: &Transform) -> Vec3 {
    transform.rotation * Vec3::NEG_Z
}

fn project_ray_to_ground(origin: Vec3, direction: Vec3) -> Option<Vec3> {
    if direction.y.abs() < f32::EPSILON {
        return None;
    }

    let distance = -origin.y / direction.y;
    Some(origin + direction * distance)
}

fn chunk_coord_from_world_xz(world_x: f32, world_z: f32, config: &WorldConfig) -> IVec2 {
    let chunk_size = config.chunk_world_size();
    IVec2::new(
        (world_x / chunk_size).floor() as i32,
        (world_z / chunk_size).floor() as i32,
    )
}

fn outside_bounds(coord: IVec2, min: IVec2, max: IVec2) -> bool {
    coord.x < min.x || coord.y < min.y || coord.x > max.x || coord.y > max.y
}

fn orthographic_view_size(ortho: &OrthographicProjection, window: &Window) -> (f32, f32) {
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
