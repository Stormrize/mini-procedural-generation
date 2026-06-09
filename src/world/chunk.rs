use bevy::prelude::IVec2;

use crate::world::generation::heightmap::HeightMap;
use crate::world::tile::Tile;
use crate::world::world_config::{CHUNK_SIZE, WorldConfig};

#[derive(Clone)]
pub struct Chunk {
    pub coord: IVec2,
    tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn generate(coord: IVec2, config: &WorldConfig) -> Self {
        let height_map = HeightMap::generate_chunk(coord, config);
        let mut tiles = [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE];

        for row in 0..CHUNK_SIZE {
            for col in 0..CHUNK_SIZE {
                let height = height_map.get(col, row);
                tiles[row][col] = Tile::new(height, config);
            }
        }

        Self { coord, tiles }
    }

    /// Получить ссылку на тайл по координатам (col, row).
    #[allow(dead_code)]
    #[inline]
    pub fn get(&self, col: usize, row: usize) -> &Tile {
        &self.tiles[row][col]
    }

    /// Получить изменяемую ссылку на тайл.
    #[allow(dead_code)]
    #[inline]
    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut Tile {
        &mut self.tiles[row][col]
    }

    /// Итератор по всем тайлам: (col, row, &Tile).
    pub fn iter_tiles(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        self.tiles.iter().enumerate().flat_map(|(row, row_tiles)| {
            row_tiles
                .iter()
                .enumerate()
                .map(move |(col, tile)| (col, row, tile))
        })
    }

    /// Итератор по всем тайлам с изменяемым доступом.
    #[allow(dead_code)]
    pub fn iter_tiles_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Tile)> {
        self.tiles
            .iter_mut()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                row_tiles
                    .iter_mut()
                    .enumerate()
                    .map(move |(col, tile)| (col, row, tile))
            })
    }

    /// Общее количество тайлов в чанке.
    #[allow(dead_code)]
    #[inline]
    pub const fn tile_count(&self) -> usize {
        CHUNK_SIZE * CHUNK_SIZE
    }

    /// Ширина чанка.
    #[allow(dead_code)]
    #[inline]
    pub const fn width() -> usize {
        CHUNK_SIZE
    }

    /// Высота чанка.
    #[allow(dead_code)]
    #[inline]
    pub const fn height() -> usize {
        CHUNK_SIZE
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            coord: IVec2::ZERO,
            tiles: [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}
