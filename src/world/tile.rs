use bevy::prelude::Entity;
use crate::world::chunk::config::WorldConfig;
use crate::world::world_enums::biome::Biome;

#[derive(Clone, Copy)]
pub struct Tile {
    // pub terrain: u8, //TODO
    // pub height: f32,
    // pub moisture: f32,
    // pub temperature: f32,
    // pub structure: Option<Entity>,
    pub biome: Biome
}

// impl Default for Tile {
//     fn default() -> Self {
//         Self {
//             terrain: 0,
//             height: 0.0,
//             moisture: 0.0,
//             temperature: 0.0,
//             structure: None,
//         }
//     }
// }

impl Tile {
    pub fn new(height: f32, cfg: &WorldConfig) -> Self {
        let biome = Biome::from_height(height, cfg);
        Tile { biome }
    }
}