//! Генерация карты высот для одного чанка.

use bevy::prelude::IVec2;
use noise::Perlin;

use super::noise::{fbm, normalise, sample};
use crate::world::world_config::{CHUNK_SIZE, IslandShape, WorldConfig, WorldShape};

/// Плоская сетка значений высот, строка-за-строкой.
pub struct HeightMap {
    pub width: usize,
    pub height: usize,
    /// Данные в row-major порядке: `data[row * width + col]`.
    /// Значения в [0.0, 1.0].
    pub data: Vec<f32>,
}

impl HeightMap {
    /// Сгенерировать карту высот для чанка. Семплирование идёт в глобальных
    /// координатах тайлов, поэтому соседние чанки стыкуются без швов.
    pub fn generate_chunk(chunk_coord: IVec2, config: &WorldConfig) -> Self {
        let perlin_main = Perlin::new(config.seed);
        let perlin_detail = Perlin::new(config.seed.wrapping_add(1337));

        let w = CHUNK_SIZE;
        let h = CHUNK_SIZE;
        let mut data = vec![0.0_f32; w * h];

        for row in 0..h {
            for col in 0..w {
                let world_col = chunk_coord.x as i64 * CHUNK_SIZE as i64 + col as i64;
                let world_row = chunk_coord.y as i64 * CHUNK_SIZE as i64 + row as i64;
                let nx = world_col as f64 * config.frequency;
                let ny = world_row as f64 * config.frequency;

                let large = fbm(
                    &perlin_main,
                    nx,
                    ny,
                    config.octaves,
                    config.persistence,
                    config.lacunarity,
                );
                let large = normalise(large);

                let detail = normalise(sample(
                    &perlin_detail,
                    world_col as f64 * config.detail_frequency,
                    world_row as f64 * config.detail_frequency,
                ));

                let combined = large * (1.0 - config.detail_weight) + detail * config.detail_weight;

                let falloff = world_falloff(world_col, world_row, &config.world_shape);
                data[row * w + col] = (combined - falloff).clamp(0.0, 1.0) as f32;
            }
        }

        Self {
            width: w,
            height: h,
            data,
        }
    }

    /// Получить высоту в клетке (col, row).
    #[inline]
    pub fn get(&self, col: usize, row: usize) -> f32 {
        debug_assert!(col < self.width);
        debug_assert!(row < self.height);
        self.data[row * self.width + col]
    }
}

fn world_falloff(world_col: i64, world_row: i64, world_shape: &WorldShape) -> f64 {
    let WorldShape::Island {
        center_tile,
        radius_tiles,
        falloff_strength,
        shape,
    } = world_shape
    else {
        return 0.0;
    };

    let radius = (*radius_tiles).max(1.0);
    let cx = (world_col - center_tile.x as i64) as f64 / radius;
    let cy = (world_row - center_tile.y as i64) as f64 / radius;

    let d = match shape {
        IslandShape::Round => (cx * cx + cy * cy).sqrt(),
        IslandShape::Elongated => ((cx / 0.65).powi(2) + cy.powi(2)).sqrt(),
        IslandShape::Jagged => cx.abs().max(cy.abs()),
    };

    // Внутри ~55% радиуса falloff почти не трогает шум, дальше плавно топит края.
    let edge_t = ((d - 0.55) / 0.45).clamp(0.0, 1.0);
    edge_t * falloff_strength
}
