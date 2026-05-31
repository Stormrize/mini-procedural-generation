use crate::world::chunk::config::WorldConfig;
use crate::world::chunk::heightmap::HeightMap;
use crate::world::tile::Tile;

pub const CHUNK_SIZE: usize = 64;

pub struct Chunk {
    tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {

    pub fn new (config: &WorldConfig) -> Self {
        // ── Шаг 1: генерация карты высот ──────────────────────────────────────
        let height_map = HeightMap::generate(config);

        // ── Шаг 2: назначаем биомы по высоте ──────────────────────────────────
        let mut tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE];

        for row in 0..config.map_height {
            for col in 0..config.map_width {
                let height = height_map.get(col, row);
                let tile = Tile::new(height, config);
                tiles[col][row] = tile;
            }
        }

        Chunk { tiles }
    }

    /// Получить ссылку на тайл по координатам (col, row).
    #[inline]
    pub fn get(&self, col: usize, row: usize) -> &Tile {
        &self.tiles[row][col]
    }

    /// Получить изменяемую ссылку на тайл.
    #[inline]
    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut Tile {
        &mut self.tiles[row][col]
    }

    /// Итератор по всем тайлам: (col, row, &Tile).
    pub fn iter_tiles(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                row_tiles
                    .iter()
                    .enumerate()
                    .map(move |(col, tile)| (col, row, tile))
            })
    }

    /// Итератор по всем тайлам с изменяемым доступом.
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
    #[inline]
    pub const fn tile_count() -> usize {
        CHUNK_SIZE * CHUNK_SIZE
    }

    /// Ширина чанка.
    #[inline]
    pub const fn width() -> usize {
        CHUNK_SIZE
    }

    /// Высота чанка.
    #[inline]
    pub const fn height() -> usize {
        CHUNK_SIZE
    }
}