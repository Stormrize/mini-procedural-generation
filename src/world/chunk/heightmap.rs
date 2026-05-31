//! Генерация карты высот.
//!
//! Шаги:
//!   1. Двухслойный fBm-шум: крупные формы + мелкие детали.
//!   2. Island-falloff: принудительный спад к краям → форма острова.
//!   3. Ренормализация в [0.0, 1.0].

use noise::Perlin;

use super::config::{IslandShape, WorldConfig};
use super::noise::{fbm, normalise, sample};

/// Плоская сетка значений высот, строка-за-строкой.
pub struct HeightMap {
    pub width:  usize,
    pub height: usize,
    /// Данные в row-major порядке: `data[row * width + col]`.
    /// Значения в [0.0, 1.0].
    pub data: Vec<f32>,
}

impl HeightMap {
    /// Сгенерировать карту высот из параметров конфигурации.
    pub fn generate(config: &WorldConfig) -> Self {
        let w = config.map_width;
        let h = config.map_height;

        // Два перлин-шума с разными seeds: один для крупных форм, другой для деталей.
        let perlin_main   = Perlin::new(config.seed);
        let perlin_detail = Perlin::new(config.seed.wrapping_add(1337));

        let mut data = vec![0.0_f32; w * h];

        for row in 0..h {
            for col in 0..w {
                // Нормализованные координаты в [0, 1].
                let nx = col as f64 / w as f64;
                let ny = row as f64 / h as f64;

                // ── Слой 1: крупные формы (континенты / острова) ─────────
                let large = fbm(
                    &perlin_main,
                    nx * config.frequency,
                    ny * config.frequency,
                    config.octaves,
                    config.persistence,
                    config.lacunarity,
                );
                let large = normalise(large); // → [0, 1]

                // ── Слой 2: мелкие детали береговой линии ────────────────
                let detail = normalise(sample(
                    &perlin_detail,
                    nx * config.detail_frequency,
                    ny * config.detail_frequency,
                ));

                // Смешиваем слои: крупный + небольшая доля деталей.
                let combined = large * (1.0 - config.detail_weight)
                             + detail * config.detail_weight;

                // ── Island falloff: края → вода ───────────────────────────
                let falloff = island_falloff(nx, ny, config.island_falloff_strength, &config.island_shape);
                let height  = (combined - falloff).max(0.0) as f32;

                data[row * w + col] = height;
            }
        }

        // Ренормализуем: пиковое значение → 1.0.
        let max = data.iter().cloned().fold(0.0_f32, f32::max);
        if max > 1e-6 {
            for v in &mut data {
                *v /= max;
            }
        }

        Self { width: w, height: h, data }
    }

    /// Получить высоту в клетке (col, row).
    #[inline]
    pub fn get(&self, col: usize, row: usize) -> f32 {
        self.data[row * self.width + col]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Island falloff
// ─────────────────────────────────────────────────────────────────────────────

/// Вычислить, насколько нужно «понизить» высоту в точке (nx, ny),
/// чтобы края карты уходили под воду.
///
/// * `nx`, `ny` — нормализованные координаты в [0, 1].
/// * Возвращает значение в [0, strength]: чем ближе к краю, тем больше.
fn island_falloff(
    nx:       f64,
    ny:       f64,
    strength: f64,
    shape:    &IslandShape,
) -> f64 {
    // Переводим в [-1, 1] — центр карты в (0, 0).
    let cx = nx * 2.0 - 1.0;
    let cy = ny * 2.0 - 1.0;

    let d = match shape {
        // Круглый: расстояние от центра по евклиду.
        IslandShape::Round => (cx * cx + cy * cy).sqrt(),

        // Вытянутый: эллипс, сжатый по оси Y.
        IslandShape::Elongated => ((cx / 0.65).powi(2) + cy.powi(2)).sqrt(),

        // Рваный: расстояние Чебышёва — квадратная/угловатая форма.
        IslandShape::Jagged => cx.abs().max(cy.abs()),
    };

    // Ограничиваем d до 1.0, потом масштабируем на strength.
    d.min(1.0) * strength
}
