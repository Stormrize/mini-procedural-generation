use crate::terrain::generation::poisson_disk::poison_disk_sampling;
use crate::terrain::world_config::{CHUNK_SIZE, WorldConfig};
use crate::terrain::terrain_enums::details::Details;
use bevy::asset::{AssetServer, Handle};
use bevy::camera::primitives::Aabb;
use bevy::math::IVec2;
use bevy::prelude::{
    Added, Children, Commands, Entity, Name, Query, ResMut, Resource, Scene, Transform, Vec3, With,
};
use bevy::scene::SceneRoot;

pub struct SpawnedModels {
    pub entities: Vec<Entity>,
    pub models: Vec<Handle<Scene>>,
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

    let positions = poison_disk_sampling(config);
    for point in positions {
        let point_chunk = point_chunk_coord(point);

        if point_chunk != coord {
            continue;
        }

        let model = asset_server.load("magic_castle.glb#Scene0");
        let entity = commands
            .spawn((
                Name::new("castle"),
                SceneRoot(model.clone()),
                Transform::from_xyz(
                    point[0] as f32 * config.tile_size,
                    2.0,
                    point[1] as f32 * config.tile_size,
                )
                .with_scale(Vec3::splat(7.0)),
            ))
            .id();

        spawned.entities.push(entity);
        spawned.models.push(model);
    }
    // if spawned.entities.len() > 0 {
    //     println!("Spawned {} models", spawned.entities.len());
    // }
    spawned
}



pub fn despawn_models(commands: &mut Commands, spawned: SpawnedModels) {
    let mut counter = 0;
    for entity in spawned.entities {
        commands.entity(entity).despawn();
        counter += 1;
    }
    if counter > 0 {
        println!("Despawned {} models", counter);
    }
}

fn point_chunk_coord(point: [f64; 2]) -> IVec2 {
    let chunk_size = CHUNK_SIZE as i32;
    IVec2::new(
        (point[0] as i32).div_euclid(chunk_size),
        (point[1] as i32).div_euclid(chunk_size),
    )
}
