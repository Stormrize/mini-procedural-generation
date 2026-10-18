use bevy::prelude::{IVec2, Res, Resource};
use std::collections::HashMap;

use crate::terrain::chunk::Chunk;
use crate::terrain::generation::heightmap::PerlinPlaceholder;
use crate::terrain::world_config::WorldConfig;

#[derive(Resource, Default)]
pub struct World {
    pub chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn chunk_or_generate(
        &mut self,
        coord: IVec2,
        config: &WorldConfig,
        perlin_placeholder: &Res<PerlinPlaceholder>,
    ) -> &Chunk {
        self.chunks
            .entry(coord)
            .or_insert_with(|| Chunk::generate(coord, config, perlin_placeholder))
    }

    pub fn unload_chunk(&mut self, coord: IVec2) {
        self.chunks.remove(&coord);
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }
}
