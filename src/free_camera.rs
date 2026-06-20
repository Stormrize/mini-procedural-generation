use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Res};
use crate::world::world_config::WorldConfig;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct FreeCamera;


fn setup_camera(mut commands: Commands, config: Res<WorldConfig>) {
    let chunk_size = config.chunk_world_size();
    let look_at = Vec3::new(chunk_size *0.5, 0., chunk_size * 0.5);
    let dist = chunk_size * 0.85;
    let camera_pos = look_at + Vec3::new(dist, dist, dist);
}