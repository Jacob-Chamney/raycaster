use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_debug)
            .add_systems(Update, (
                display_performance_info,
                log_startup_complete,
            ));
    }
}

#[derive(Resource)]
struct DebugState {
    startup_logged: bool,
    last_info_time: f32,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            startup_logged: false,
            last_info_time: 0.0,
        }
    }
}

fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugState::default());
    info!("Debug system initialized");
}

fn display_performance_info(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut debug_state: ResMut<DebugState>,
) {
    let current_time = time.elapsed_secs();
    
    if current_time - debug_state.last_info_time >= 5.0 {
        debug_state.last_info_time = current_time;
        
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_value) = fps_diagnostic.smoothed() {
                info!("Performance - FPS: {:.1}", fps_value);
            }
        }
        
        if let Some(frame_time_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(frame_time_ms) = frame_time_diagnostic.smoothed() {
                info!("Performance - Frame Time: {:.2}ms", frame_time_ms * 1000.0);
            }
        }
        
        info!("Runtime: {:.1}s", current_time);
        info!("Press [F1] for controls help");
    }
}

fn log_startup_complete(
    time: Res<Time>,
    mut debug_state: ResMut<DebugState>,
) {
    if !debug_state.startup_logged && time.elapsed_secs() > 0.1 {
        debug_state.startup_logged = true;
        
        info!("Raycaster startup complete!");
        info!("Ready for pixel manipulation and raycasting");
        info!("Learning Phase: Plugin Architecture & Pixel Canvas");
        info!("Next Phase: Vector Math & Line Drawing");
        
        info!("Try these controls:");
        info!("   [Space] - Random pixels");
        info!("   [C] - Clear canvas");  
        info!("   [R/G/B] - Colored patterns");
        info!("   [F1] - Full help");
    }
}