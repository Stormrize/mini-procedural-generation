//! Конфигурация генерации мира.
//!
//! Все параметры в одном месте — меняй и перезапускай (R / Space),
//! чтобы сразу видеть результат.

use bevy::prelude::*;

/// Главный ресурс конфигурации. Вставляется в App один раз.
#[derive(Resource, Clone, Debug)]
pub struct WorldConfig {
    // ── Детерминизм ────────────────────────────────────────────────────────
    /// Один и тот же seed → одинаковый мир при каждом запуске.
    pub seed: u32,

    // ── Размер карты ───────────────────────────────────────────────────────
    /// Количество тайлов по горизонтали (колонки).
    pub map_width: usize,
    /// Количество тайлов по вертикали/глубине (строки, ось Z).
    pub map_height: usize,
    /// Визуальный размер одного тайла в мировых единицах.
    pub tile_size: f32,

    // ── Параметры шума (крупная форма) ─────────────────────────────────────
    /// Базовая частота шума.
    /// 0.5–1.0 → огромные континенты.
    /// 2.0–3.0 → средние острова.
    /// 4.0–8.0 → архипелаги / мелкие острова.
    pub frequency: f64,
    /// Количество октав (слоёв шума). Больше → рваные береговые линии.
    pub octaves: u32,
    /// Насколько каждый следующий октав тише предыдущего (0.0–1.0).
    /// 0.5 — классика. Выше → больше мелких деталей.
    pub persistence: f64,
    /// Насколько каждый следующий октав чаще предыдущего.
    /// 2.0 — стандарт.
    pub lacunarity: f64,

    // ── Второй слой шума (мелкие детали береговой линии) ──────────────────
    /// Частота второго (детального) слоя шума.
    pub detail_frequency: f64,
    /// Вес второго слоя (0.0 — выключен, 0.3 — заметный).
    pub detail_weight: f64,

    // ── Форма острова (island falloff) ─────────────────────────────────────
    /// Сила «принудительного» океана у краёв карты.
    /// 1.0–1.5 → умеренно. 2.0+ → почти ничего кроме острова по центру.
    pub island_falloff_strength: f64,
    /// Форма острова.
    pub island_shape: IslandShape,

    // ── Пороги высот для биомов (0.0–1.0) ─────────────────────────────────
    pub threshold_deep_water:    f32,
    pub threshold_shallow_water: f32,
    pub threshold_sand:          f32,
    pub threshold_grass:         f32,
    pub threshold_forest:        f32,
    pub threshold_hills:         f32,
    pub threshold_mountain:      f32,
    // выше threshold_mountain → Peak
}

/// Математическая форма спада высоты к краям карты.
#[derive(Clone, Debug, Default)]
pub enum IslandShape {
    /// Круглый остров (евклидово расстояние от центра).
    #[default]
    Round,
    /// Вытянутый острова (эллипс, растянутый по X).
    Elongated,
    /// Квадратный / рваный (расстояние Чебышёва).
    Jagged,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: 42,

            map_width:  64,
            map_height: 64,
            tile_size:  1.0,

            // Средний остров с интересной береговой линией.
            frequency:   6.0,
            octaves:     6,
            persistence: 0.5,
            lacunarity:  2.0,

            // Лёгкий второй слой деталей.
            detail_frequency: 8.0,
            detail_weight:    0.15,

            island_falloff_strength: 0.2,
            island_shape: IslandShape::Round,

            threshold_deep_water:    0.30,
            threshold_shallow_water: 0.40,
            threshold_sand:          0.46,
            threshold_grass:         0.62,
            threshold_forest:        0.70,
            threshold_hills:         0.78,
            threshold_mountain:      0.88,
            // >= 0.88 → Peak
        }
    }
}
