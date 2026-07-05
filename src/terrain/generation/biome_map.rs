use rayon::iter::ParallelIterator;
use rayon::iter::IndexedParallelIterator;
use bevy::math::IVec2;
use bevy::prelude::Res;
use rayon::prelude::ParallelSliceMut;
use crate::terrain::generation::heightmap::PerlinPlaceholder;
use crate::terrain::generation::noise::{fbm, normalise};
use crate::terrain::world_config::{WorldConfig, CHUNK_SIZE};

pub struct BiomeMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl BiomeMap {
    pub fn generate_biome(chunk_coord: IVec2, config: &WorldConfig, perlin_placeholder: &Res<PerlinPlaceholder>) -> Self {
        let w = CHUNK_SIZE;
        let h = CHUNK_SIZE;

        let mut data = vec![0_u8; w * h];

        data.par_chunks_mut(w)
            .enumerate()
            .for_each(|(row, row_data)| {
                for col in 0..w {
                    let world_col = chunk_coord.x * CHUNK_SIZE as i32 + col as i32;
                    let world_row = chunk_coord.y * CHUNK_SIZE as i32 + row as i32;

                    let nx = world_col as f64 * config.frequency * 3.0;
                    let ny = world_row as f64 * config.frequency * 1.0;

                    let large = fbm(
                        &perlin_placeholder.perlin_main,
                        nx,
                        ny,
                        2,
                        config.persistence,
                        config.lacunarity,
                    );
                    let large = normalise(large);
                    row_data[col] = (large.clamp(0.0, 1.0) * 255.0) as u8 ;
                }
            }
        );

        Self {
            width: w,
            height: h,
            data,
        }
    }

    #[inline]
    pub fn get(&self, col: usize, row: usize) -> u8 {
        debug_assert!(col < self.width);
        debug_assert!(row < self.height);
        self.data[row * self.width + col]
    }
}