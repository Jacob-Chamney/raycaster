use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_TITLE: &str = "Rust Raycaster Learning Project";

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Camera spawned for raycaster display");
    info!("Window configured: {}x{}", WINDOW_WIDTH, WINDOW_HEIGHT);
}