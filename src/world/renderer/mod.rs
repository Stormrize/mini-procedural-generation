//! WorldPlugin: конфиг, стриминг чанков и реакция на regenerate.

pub mod chunk_renderer;
pub mod camera_math;
pub mod events;
pub mod chunk_straming;
mod details_renderer;

use bevy::prelude::*;
use std::collections::HashMap;

use crate::world::renderer::events::{handle_input, RegenerateWorldEvent};
use crate::world::renderer::chunk_straming::stream_chunks;
use crate::world::renderer::chunk_renderer::{
    despawn_chunk_meshes, SpawnedChunkMeshes, TileAssets,
};
use crate::world::renderer::details_renderer::{auto_scale_system, despawn_models, SpawnedModels};
use crate::world::world::World;
use crate::world::world_config::WorldConfig;

#[derive(Resource, Default)]
pub struct Rendered {
    chunks: HashMap<IVec2, SpawnedChunkMeshes>,
    details: HashMap<IVec2, SpawnedModels>,
}



pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default())
            .insert_resource(World::default())
            .insert_resource(Rendered::default())
            .add_message::<RegenerateWorldEvent>()
            .add_systems(Startup, setup_tile_assets)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks).chain(),
            );
    }
}

fn setup_tile_assets(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(TileAssets::build(&mut materials));
}

fn apply_regenerate(
    mut events: MessageReader<RegenerateWorldEvent>,
    mut commands: Commands,
    mut world: ResMut<World>,
    mut rendered: ResMut<Rendered>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if events.read().count() == 0 {
        return;
    }

    for (_, chunk_meshes) in rendered.chunks.drain() {
        despawn_chunk_meshes(&mut commands, &mut meshes, chunk_meshes);
    }
    for (_, models) in rendered.details.drain() {
        despawn_models(&mut commands, models);
    }
    world.clear();
}
