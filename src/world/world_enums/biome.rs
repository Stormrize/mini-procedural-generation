//! Определения биомов.
//!
//! Биомы назначаются по высоте (этап 2).
//! Порядок от самого низкого к высокому:
//!   DeepWater → ShallowWater → Sand → Grass → Forest → Hills → Mountain → Peak

use bevy::prelude::Color;

use crate::world::chunk::config::WorldConfig;

/// Все возможные биомы тайла.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Biome {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Hills,
    Mountain,
    Peak,
}

impl Biome {
    /// Определить биом по нормализованной высоте (0.0–1.0).
    pub fn from_height(height: f32, cfg: &WorldConfig) -> Self {
        match height {
            h if h < cfg.threshold_deep_water    => Biome::DeepWater,
            h if h < cfg.threshold_shallow_water => Biome::ShallowWater,
            h if h < cfg.threshold_sand          => Biome::Sand,
            h if h < cfg.threshold_grass         => Biome::Grass,
            h if h < cfg.threshold_forest        => Biome::Forest,
            h if h < cfg.threshold_hills         => Biome::Hills,
            h if h < cfg.threshold_mountain      => Biome::Mountain,
            _                                    => Biome::Peak,
        }
    }

    /// Визуальная высота бокса тайла в мировых единицах.
    ///
    /// Вода — плоская, пики — высокие.
    /// Используется как scale.y при рендеринге.
    pub fn visual_height(self) -> f32 {
        match self {
            Biome::DeepWater    => 0.08,
            Biome::ShallowWater => 0.14,
            Biome::Sand         => 0.20,
            Biome::Grass        => 0.26,
            Biome::Forest       => 0.35,
            Biome::Hills        => 0.60,
            Biome::Mountain     => 1.20,
            Biome::Peak         => 2.00,
        }
    }

    /// Плейсхолдер-цвет до загрузки реальных текстур.
    ///
    /// TODO: замени на `base_color_texture: Some(asset_server.load("..."))` в renderer.rs
    pub fn color(self) -> Color {
        match self {
            Biome::DeepWater    => Color::srgb(0.04, 0.16, 0.44),
            Biome::ShallowWater => Color::srgb(0.14, 0.42, 0.72),
            Biome::Sand         => Color::srgb(0.87, 0.80, 0.52),
            Biome::Grass        => Color::srgb(0.27, 0.62, 0.20),
            Biome::Forest       => Color::srgb(0.10, 0.36, 0.10),
            Biome::Hills        => Color::srgb(0.58, 0.50, 0.30),
            Biome::Mountain     => Color::srgb(0.52, 0.48, 0.46),
            Biome::Peak         => Color::srgb(0.93, 0.96, 0.98),
        }
    }

    /// Порядковый индекс биома для массива материалов (0–7).
    pub fn index(self) -> usize {
        match self {
            Biome::DeepWater    => 0,
            Biome::ShallowWater => 1,
            Biome::Sand         => 2,
            Biome::Grass        => 3,
            Biome::Forest       => 4,
            Biome::Hills        => 5,
            Biome::Mountain     => 6,
            Biome::Peak         => 7,
        }
    }

    /// Все биомы по порядку (для итерации при создании материалов).
    pub const ALL: [Biome; 8] = [
        Biome::DeepWater,
        Biome::ShallowWater,
        Biome::Sand,
        Biome::Grass,
        Biome::Forest,
        Biome::Hills,
        Biome::Mountain,
        Biome::Peak,
    ];
}
