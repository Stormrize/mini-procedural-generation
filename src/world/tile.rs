use crate::world::world_config::WorldConfig;
use crate::world::world_enums::biome::Biome;

#[derive(Clone, Copy)]
pub struct Tile {
    pub height: f32,
    #[allow(dead_code)]
    pub moisture: f32,
    #[allow(dead_code)]
    pub temperature: f32,
    pub biome: Biome,
}

impl Tile {
    pub fn new(height: f32, cfg: &WorldConfig) -> Self {
        let biome = Biome::from_height(height, cfg);
        Tile {
            height,
            moisture: 0.0,
            temperature: 0.0,
            biome,
        }
    }

    #[inline]
    pub fn visual_height(self) -> f32 {
        self.biome.visual_height() + self.height * 0.04
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            height: 0.0,
            moisture: 0.0,
            temperature: 0.0,
            biome: Biome::default(),
        }
    }
}
