use bevy::prelude::Color;

use crate::terrain::world_config::WorldConfig;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Biome {
    DeepWater,
    ShallowWater,
    Sand,

    #[default]
    Grass,

    Forest,
    Hills,
    Mountain,
    Peak,
}

impl Biome {
    pub fn from_height(height: u8, cfg: &WorldConfig) -> Self {
        match height {
            h if h < cfg.threshold_deep_water => Biome::DeepWater,
            h if h < cfg.threshold_shallow_water => Biome::ShallowWater,
            h if h < cfg.threshold_sand => Biome::Sand,
            h if h < cfg.threshold_grass => Biome::Grass,
            h if h < cfg.threshold_forest => Biome::Forest,
            h if h < cfg.threshold_hills => Biome::Hills,
            h if h < cfg.threshold_mountain => Biome::Mountain,
            _ => Biome::Peak,
        }
    }

    pub fn visual_height(self) -> u8 {
        match self {
            Biome::DeepWater => 10,    //0.08
            Biome::ShallowWater => 18, //0.14
            Biome::Sand => 25,         //0.20
            Biome::Grass => 33,        //0.26
            Biome::Forest => 45,       //0.35
            Biome::Hills => 77,        //0.60
            Biome::Mountain => 154,
            Biome::Peak => 255,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Biome::DeepWater => Color::srgb(0.04, 0.16, 0.44),
            Biome::ShallowWater => Color::srgb(0.14, 0.42, 0.72),
            Biome::Sand => Color::srgb(0.87, 0.80, 0.52),
            Biome::Grass => Color::srgb(0.27, 0.62, 0.20),
            Biome::Forest => Color::srgb(0.10, 0.36, 0.10),
            Biome::Hills => Color::srgb(0.58, 0.50, 0.30),
            Biome::Mountain => Color::srgb(0.52, 0.48, 0.46),
            Biome::Peak => Color::srgb(0.93, 0.96, 0.98),
        }
    }

    /// Порядковый индекс биома для массива материалов (0–7).
    pub fn index(self) -> usize {
        match self {
            Biome::DeepWater => 0,
            Biome::ShallowWater => 1,
            Biome::Sand => 2,
            Biome::Grass => 3,
            Biome::Forest => 4,
            Biome::Hills => 5,
            Biome::Mountain => 6,
            Biome::Peak => 7,
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
