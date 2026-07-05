use crate::terrain::world_config::WorldConfig;
use crate::terrain::terrain_enums::biome::Biome;

#[derive(Clone, Copy)]
pub struct Tile {
    pub height: u8,
    #[allow(dead_code)]
    pub moisture: u8,
    #[allow(dead_code)]
    pub temperature: u8,
    pub biome: Biome,
}

impl Tile {
    pub fn new(height: u8, cfg: &WorldConfig) -> Self {
        let biome = Biome::from_height(height, cfg);
        Tile {
            height,
            moisture: 0,
            temperature: 0,
            biome,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            height: 0,
            moisture: 0,
            temperature: 0,
            biome: Biome::default(),
        }
    }
}
