use bevy::asset::{AssetServer, Handle};
use bevy::camera::primitives::Aabb;
use bevy::math::IVec2;
use bevy::prelude::{Added, Children, Commands, Entity, Name, Query, ResMut, Resource, Scene, Transform, Vec3, With};
use bevy::scene::SceneRoot;

use crate::world::generation::voronoi::random_points;
use crate::world::world_config::{WorldConfig, CHUNK_SIZE};
use crate::world::world_enums::details::Details;

pub struct SpawnedModels {
    pub entities: Vec<Entity>,
    pub models: Vec<Handle<Scene>>,
}

#[derive(Resource, Clone)]
pub struct Points {
    pub points: Vec<IVec2>,
}

pub fn spawn_models(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    config: &WorldConfig,
    coord: IVec2,
) -> SpawnedModels {

    let mut spawned = SpawnedModels {
        entities: Vec::with_capacity(Details::ALL.len()),
        models: Vec::with_capacity(Details::ALL.len()),
    };

    let positions = random_points(config);
    for point in positions {
        let point_chunk = point_chunk_coord(point);

        if point_chunk != coord {
            continue;
        }

        let model = asset_server.load("cultist_low_poly.glb#Scene0");
        let entity = commands
            .spawn((
                Name::new("tower-top"),
                SceneRoot(model.clone()),
                Transform::from_xyz(
                    point.x as f32 * config.tile_size,
                    2.0,
                    point.y as f32 * config.tile_size,
                ).with_scale(Vec3::splat(6.0)),
            ))
            .id();

        spawned.entities.push(entity);
        spawned.models.push(model);
    }
    if spawned.entities.len() > 0 {
        println!("Spawned {} models", spawned.entities.len());
    }
    spawned
}

pub fn auto_scale_system(
    mut commands: Commands,
    // Ищем детей (меши) внутри нашего здания
    children_query: Query<&Children, With<Name>>,
    // Ищем Aabb (размеры) у вложенных мешей
    aabb_query: Query<(Entity, &Aabb), Added<Aabb>>,
) {
    for children in &children_query {
        for &child in children.iter() {
            if let Ok((entity, aabb)) = aabb_query.get(child) {
                let current_height = aabb.half_extents.y * 2.0;

                if current_height > 0.0 {
                    let scale_factor = 5.0 / current_height;

                    commands.entity(entity).insert(Transform::from_scale(Vec3::splat(scale_factor)));

                    println!("Здание отмасштабировано. Исходная высота была: {:.2} м.", current_height);
                }
            }
        }
    }
}

pub fn despawn_models(
    commands: &mut Commands,
    spawned: SpawnedModels,
) {
    let mut counter = 0;
    for entity in spawned.entities {
        commands.entity(entity).despawn();
        counter +=1;
    }
    if counter > 0 {
        println!("Despawned {} models", counter);
    }
}

fn point_chunk_coord(point: crate::world::generation::voronoi::Point) -> IVec2 {
    let chunk_size = CHUNK_SIZE as i32;
    IVec2::new(
        point.x.div_euclid(chunk_size),
        point.y.div_euclid(chunk_size),
    )
}
