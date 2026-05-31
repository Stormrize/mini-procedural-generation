use std::collections::HashMap;
use bevy::prelude::{Component, IVec2};
use crate::world::chunks::Chunk;

#[derive(Component)]
pub struct World {
    pub chunks: HashMap<IVec2, Chunk>
}