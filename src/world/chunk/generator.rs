//! Генератор карты.
//!
//! Чистая функция: WorldConfig → MapData.
//! Никаких Bevy ECS-зависимостей — легко тестировать изолированно.
//!
//! Порядок шагов:
//!   1. HeightMap::generate  — двухслойный шум + island falloff
//!   2. Биомы из высоты      — Biome::from_height

use super::config::WorldConfig;
use super::heightmap::HeightMap;
use crate::world::chunks::{Chunk, CHUNK_SIZE};
use crate::world::tile::Tile;
use crate::world::world_enums::biome::Biome;

/// Сгенерировать карту из конфигурации и вернуть `Chunk`.
///
/// Эта функция вызывается как при старте, так и при перегенерации.
pub fn generate_map_data(config: &WorldConfig) -> Chunk {
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

    Chunk::new(&tiles)
}
