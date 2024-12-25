use bevy::{
    prelude::*,
    render::{render_resource::Face, view::NoFrustumCulling},
    window::{
        MonitorSelection, Window, WindowMode, WindowPlugin, WindowPosition, WindowResolution,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(1920., 1080.),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        })
        .run();
}
