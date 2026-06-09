use bevy::prelude::{IVec2, Resource};
use std::collections::HashMap;

use crate::world::chunk::Chunk;
use crate::world::world_config::WorldConfig;

#[derive(Resource, Default)]
pub struct World {
    pub chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn chunk_or_generate(&mut self, coord: IVec2, config: &WorldConfig) -> &Chunk {
        self.chunks
            .entry(coord)
            .or_insert_with(|| Chunk::generate(coord, config))
    }

    pub fn unload_chunk(&mut self, coord: IVec2) {
        self.chunks.remove(&coord);
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }
}
