use bevy::prelude::*;
use super::canvas::PixelCanvas;
use super::player::Player;
use super::map::GameMap;

#[derive(Resource)]
pub struct RenderSettings {
    pub show_minimap: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            show_minimap: true,
        }
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RenderSettings::default())
            .add_systems(Update, (
                toggle_minimap,
                render_minimap,
            ));
    }
}

fn toggle_minimap(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<RenderSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        settings.show_minimap = !settings.show_minimap;
        info!("Minimap: {}", if settings.show_minimap { "ON" } else { "OFF" });
    }
}

fn render_minimap(
    mut canvas: ResMut<PixelCanvas>,
    player: Res<Player>,
    map: Option<Res<GameMap>>,
    settings: Res<RenderSettings>,
) {
    if !settings.show_minimap {
        return;
    }
    
    let Some(map) = map else { return };
    
    let minimap_size = 100;
    let minimap_scale = 4;
    let start_x = (canvas.width - minimap_size) as i32;
    let start_y = 10i32;
    
    // Clear minimap area with border
    for y in 0..minimap_size + 2 {
        for x in 0..minimap_size + 2 {
            let px = (start_x - 1 + x as i32) as u32;
            let py = (start_y - 1 + y as i32) as u32;
            if px < canvas.width && py < canvas.height {
                canvas.set_pixel(px, py, [0, 0, 0, 255]);
            }
        }
    }
    
    // Draw map tiles
    for map_y in 0..map.height {
        for map_x in 0..map.width {
            let tile = map.get_tile(map_x, map_y);
            let color = match tile {
                0 => [40, 40, 40, 255],   // Floor - dark gray
                1 => [255, 255, 255, 255], // Wall - white
                2 => [0, 255, 0, 255],     // Green wall
                3 => [0, 0, 255, 255],     // Blue wall
                4 => [255, 255, 0, 255],   // Yellow wall
                5 => [255, 0, 255, 255],   // Magenta wall
                _ => [128, 128, 128, 255], // Unknown - gray
            };
            
            let pixel_x = start_x + (map_x * minimap_scale / map.width) as i32;
            let pixel_y = start_y + (map_y * minimap_scale / map.height) as i32;
            
            if pixel_x >= 0 && pixel_y >= 0 {
                canvas.set_pixel(pixel_x as u32, pixel_y as u32, color);
            }
        }
    }
    
    // Draw player position
    let player_x = start_x + (player.position.x * minimap_scale as f32 / map.width as f32) as i32;
    let player_y = start_y + (player.position.y * minimap_scale as f32 / map.height as f32) as i32;
    
    if player_x >= 0 && player_y >= 0 {
        canvas.set_pixel(player_x as u32, player_y as u32, [255, 0, 0, 255]); // Red dot for player
    }
}