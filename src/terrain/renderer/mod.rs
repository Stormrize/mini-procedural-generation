//! WorldPlugin: конфиг, стриминг чанков и реакция на regenerate.

pub mod camera_math;
pub mod chunk_renderer;
pub mod chunk_straming;
use crate::terrain::generation::heightmap::PerlinPlaceholder;
use crate::terrain::renderer::chunk_renderer::{
    SpawnedChunkMeshes, TileAssets, despawn_chunk_meshes,
};
use crate::terrain::renderer::chunk_straming::stream_chunks;
use crate::input::events::{RegenerateWorldEvent, handle_input};
use bevy::prelude::*;
use std::collections::HashMap;

use crate::terrain::world::World;
use crate::terrain::world_config::WorldConfig;

#[derive(Resource, Default)]
pub struct Rendered {
    chunks: HashMap<IVec2, SpawnedChunkMeshes>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        println!("terrain plugin 1");
        app.insert_resource(WorldConfig::default())
            .insert_resource(PerlinPlaceholder::default())
            .insert_resource(World::default())
            .insert_resource(Rendered::default())
            .add_message::<RegenerateWorldEvent>()
            .add_systems(Startup, setup_tile_assets)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks).chain(),
            );
        println!("terrain plugin 2");
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
    world.clear();
}
