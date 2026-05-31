//! Утилиты для работы с шумом.
//!
//! Все вызовы `noise`-крейта изолированы здесь — остальной код
//! не знает о конкретной библиотеке шума.

use noise::{NoiseFn, Perlin};

// ─────────────────────────────────────────────────────────────────────────────
// fBm — Fractal Brownian Motion
// ─────────────────────────────────────────────────────────────────────────────

/// Многооктавный (фрактальный) шум Перлина в точке (x, y).
///
/// Возвращает значение примерно в диапазоне [−1, 1].
/// Используй [`normalise`] для перевода в [0, 1].
pub fn fbm(
    perlin:      &Perlin,
    x:           f64,
    y:           f64,
    octaves:     u32,
    persistence: f64,  // насколько каждый слой тише предыдущего
    lacunarity:  f64,  // насколько каждый слой чаще предыдущего
) -> f64 {
    let mut value     = 0.0_f64;
    let mut amplitude = 1.0_f64;
    let mut frequency = 1.0_f64;
    let mut max_amp   = 0.0_f64; // для нормализации в конце

    for _ in 0..octaves {
        value   += perlin.get([x * frequency, y * frequency]) * amplitude;
        max_amp += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    // Нормализуем, чтобы итоговый диапазон оставался в [-1, 1].
    value / max_amp
}

/// Перевод из [−1, 1] → [0, 1].
#[inline]
pub fn normalise(v: f64) -> f64 {
    (v + 1.0) * 0.5
}

/// Одиночный шум (без октав) для быстрых деталей.
#[inline]
pub fn sample(perlin: &Perlin, x: f64, y: f64) -> f64 {
    perlin.get([x, y])
}
