mod camera;
mod world;


use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Procedural World — Stage 1–2".to_string(),
                    resolution: (1280, 720).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::chunk::WorldPlugin)
        .run();
}
