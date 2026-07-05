//! Конфигурация генерации и стриминга мира.

use std::num::FpCategory::Infinite;
use bevy::prelude::{IVec2, Resource};
use noise::Perlin;

/// Размер чанка фиксирован, чтобы `Chunk` хранил тайлы в плотном массиве.
pub const CHUNK_SIZE: usize = 64;

/// Глобальные размеры мира в тайлах для heightmap falloff и Voronoi-генерации.
pub const WORLD_WIDTH: usize = CHUNK_SIZE * 12;
pub const WORLD_HEIGHT: usize = CHUNK_SIZE * 12;


/// Приближение камеры
pub const MIN_ORTHOGRAPHIC_SCALE: f32 = 0.1;
pub const MAX_ORTHOGRAPHIC_SCALE: f32 = 900.0;

/// Главный ресурс конфигурации. Вставляется в App один раз.
#[derive(Resource, Clone)]
pub struct WorldConfig {
    pub seed: u32,

    pub tile_size: f32,
    pub preload_margin_chunks: i8,
    pub unload_margin_chunks: i8,
    pub camera_viewport_height: f32,

    pub octaves: u8,
    pub frequency: f64,
    pub persistence: f64,
    pub lacunarity: f64,
    pub detail_frequency: f64,
    pub detail_weight: f64,

    pub world_shape: WorldShape,

    pub threshold_deep_water: u8,
    pub threshold_shallow_water: u8,
    pub threshold_sand: u8,
    pub threshold_grass: u8,
    pub threshold_forest: u8,
    pub threshold_hills: u8,
    pub threshold_mountain: u8,
    // выше threshold_mountain → Peak

    pub human_threshold: u8,
    pub elf_threshold: u8,
}
impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            tile_size: 16.0,
            preload_margin_chunks: 1,
            unload_margin_chunks: 14,
            camera_viewport_height: CHUNK_SIZE as f32 * 1.5,

            // Низкие частоты важны для чанков: шум семплируется в глобальных tile-координатах.
            frequency: 0.018,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,

            detail_frequency: 0.075,
            detail_weight: 0.15,

            world_shape: WorldShape::Island {
                center_tile: IVec2::ZERO,
                width_tiles: WORLD_WIDTH as u32,
                height_tiles: WORLD_HEIGHT as u32,
                falloff_strength: 0.45,
                shape: IslandShape::Jagged,
            },

            threshold_deep_water: 76,     //0.3
            threshold_shallow_water: 102, //0.4
            threshold_sand: 117,          //0.46
            threshold_grass: 158,         //0.62
            threshold_forest: 178,        //0.7
            threshold_hills: 199,         //0.78
            threshold_mountain: 224,      //0.88
                                          // >= 0.88 → Peak

            human_threshold: 75,
            elf_threshold: 150,
        }
    }
}

impl WorldConfig {
    #[inline]
    pub fn chunk_world_size(&self) -> f32 {
        CHUNK_SIZE as f32 * self.tile_size
    }
}

/// Глобальная форма мира. `Island` задает маску острова поверх чанкового мира.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum WorldShape {
    #[default]
    Infinite,
    Island {
        center_tile: IVec2,
        width_tiles: u32,
        height_tiles: u32,
        falloff_strength: f64,
        shape: IslandShape,
    },
}

/// Математическая форма спада высоты к краям карты.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum IslandShape {
    /// Круглый остров (евклидово расстояние от центра).
    #[default]
    Round,
    /// Квадратный / рваный (расстояние Чебышёва).
    Jagged,
}
