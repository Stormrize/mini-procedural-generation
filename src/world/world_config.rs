//! Конфигурация генерации и стриминга мира.

use bevy::prelude::{IVec2, Resource};

/// Размер чанка фиксирован, чтобы `Chunk` хранил тайлы в плотном массиве.
pub const CHUNK_SIZE: usize = 64;

/// Главный ресурс конфигурации. Вставляется в App один раз.
#[derive(Resource, Clone)]
pub struct WorldConfig {
    /// Один и тот же seed → одинаковый мир при каждом запуске.
    pub seed: u32,

    /// Визуальный размер одного тайла в мировых единицах.
    pub tile_size: f32,

    /// Сколько чанков гарантированно держать вокруг камеры.
    pub min_loaded_radius_chunks: i32,
    /// Жёсткий лимит, чтобы сильный zoom-out не создал сотни чанков за кадр.
    pub max_loaded_radius_chunks: i32,
    /// Дополнительные чанки вокруг видимой области, чтобы не было подгрузки на самой границе экрана.
    pub preload_margin_chunks: i32,
    /// Через сколько чанков за пределами радиуса выгружать данные и mesh.
    pub unload_margin_chunks: i32,
    /// Максимум новых чанков, которые можно сгенерировать и заспавнить за один кадр.
    pub max_chunk_loads_per_frame: usize,

    /// Стартовая высота ортографического viewport в мировых единицах.
    pub camera_viewport_height: f32,

    /// Базовая частота height-шумa в глобальных координатах тайлов.
    /// Меньше значение → крупнее формы континентов.
    pub frequency: f64,
    /// Количество октав. Больше → больше деталей, но дороже генерация чанка.
    pub octaves: u32,
    /// Насколько каждый следующий октав тише предыдущего.
    pub persistence: f64,
    /// Насколько каждый следующий октав чаще предыдущего.
    pub lacunarity: f64,
    /// Частота второго слоя шума для мелких деталей береговой линии.
    pub detail_frequency: f64,
    /// Вес второго слоя.
    pub detail_weight: f64,

    /// Глобальная форма мира. По умолчанию бесконечная/чанковая без island falloff.
    pub world_shape: WorldShape,

    pub threshold_deep_water: f32,
    pub threshold_shallow_water: f32,
    pub threshold_sand: f32,
    pub threshold_grass: f32,
    pub threshold_forest: f32,
    pub threshold_hills: f32,
    pub threshold_mountain: f32,
    // выше threshold_mountain → Peak
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: 42,

            tile_size: 1.0,

            min_loaded_radius_chunks: 1,
            max_loaded_radius_chunks: 5,
            preload_margin_chunks: 1,
            unload_margin_chunks: 1,
            max_chunk_loads_per_frame: 8,

            camera_viewport_height: CHUNK_SIZE as f32 * 1.5,

            // Низкие частоты важны для чанков: шум семплируется в глобальных tile-координатах.
            frequency: 0.018,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,

            detail_frequency: 0.075,
            detail_weight: 0.15,

            world_shape: WorldShape::Infinite,

            threshold_deep_water: 0.30,
            threshold_shallow_water: 0.40,
            threshold_sand: 0.46,
            threshold_grass: 0.62,
            threshold_forest: 0.70,
            threshold_hills: 0.78,
            threshold_mountain: 0.88,
            // >= 0.88 → Peak
        }
    }
}

impl WorldConfig {
    #[inline]
    pub fn chunk_world_size(&self) -> f32 {
        CHUNK_SIZE as f32 * self.tile_size
    }
}

/// Глобальная форма мира. `Island` оставлен как режим для старой идеи с островом,
/// но default теперь подходит для бесконечного чанкового мира.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum WorldShape {
    #[default]
    Infinite,
    Island {
        center_tile: IVec2,
        radius_tiles: f64,
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
    /// Вытянутый острова (эллипс, растянутый по X).
    Elongated,
    /// Квадратный / рваный (расстояние Чебышёва).
    Jagged,
}
