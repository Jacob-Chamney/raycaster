use bevy::prelude::*;

mod plugins;
use plugins::{
    window::{WindowPlugin as RaycasterWindowPlugin, WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE},
    canvas::CanvasPlugin,
    input::InputPlugin as RaycasterInputPlugin,
    debug::DebugPlugin,
    math::MathPlugin,
    player::PlayerPlugin,
    map::MapPlugin,
    raycast::RaycastPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            RaycasterWindowPlugin,
            MathPlugin,
            MapPlugin,
            PlayerPlugin,
            CanvasPlugin,
            RaycastPlugin,
            RaycasterInputPlugin,
            DebugPlugin,
        ))
        .run();
}