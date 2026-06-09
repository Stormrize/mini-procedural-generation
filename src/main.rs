mod camera;
mod world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Procedural World — Stage 1–2".to_string(),

                //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::renderer::WorldPlugin)
        .run();
}
