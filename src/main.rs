use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode, WindowMode},
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod circle;
use circle::CirclePlugin;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                title: "gaming".into(),
                present_mode: PresentMode::AutoNoVsync,
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(CirclePlugin)
        .add_systems(Update, close_on_esc)
        .run();
}
