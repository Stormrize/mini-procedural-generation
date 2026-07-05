//25 lod 10 lod

use bevy::prelude::Res;
use crate::camera::camera::CameraZoom;

pub fn lod(zoom: Res<CameraZoom>) {
    if zoom.zoom < 25.0 {
        
    }
    if zoom.zoom < 10.0 {
        
    }
}

fn split_tile() {
    
}