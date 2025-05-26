use bevy::prelude::*;
use super::canvas::{PixelCanvas, CANVAS_WIDTH, CANVAS_HEIGHT};
use super::player::Player;
use super::map::GameMap;
use super::math::Vec2f;

pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_3d_view);
    }
}

struct RayHit {
    distance: f32,
    wall_type: u8,
    side: bool, // false = NS wall, true = EW wall
}

fn render_3d_view(
    mut canvas: ResMut<PixelCanvas>,
    player: Res<Player>,
    map: Res<GameMap>,
) {
    canvas.clear([135, 206, 235, 255]); // Sky blue
    
    let screen_width = CANVAS_WIDTH as f32;
    let screen_height = CANVAS_HEIGHT as f32;
    
    // Calculate vertical offset from pitch - clamp to prevent overflow
    let pitch_offset = (player.pitch * screen_height * 0.3).clamp(-200.0, 200.0) as i32;
    let horizon = ((screen_height / 2.0) as i32 + pitch_offset).clamp(0, screen_height as i32 - 1);
    
    // Cast rays for each vertical line on screen
    for x in 0..CANVAS_WIDTH {
        let camera_x = 2.0 * x as f32 / screen_width - 1.0;
        let ray_dir = Vec2f::new(
            player.direction.x + player.plane.x * camera_x,
            player.direction.y + player.plane.y * camera_x,
        );
        
        if let Some(hit) = cast_ray(&player.position, ray_dir, &map) {
            let line_height = ((screen_height / hit.distance.max(0.01)) as i32).min(screen_height as i32 * 2);
            let wall_half = line_height / 2;
            
            let draw_start = (horizon - wall_half).max(0).min(screen_height as i32 - 1) as u32;
            let draw_end = (horizon + wall_half).max(0).min(screen_height as i32 - 1) as u32;
            
            let wall_color = get_wall_color(hit.wall_type, hit.side);
            
            // Draw floor (only if there's space below the wall)
            if draw_end < CANVAS_HEIGHT - 1 {
                for y in (draw_end + 1)..CANVAS_HEIGHT {
                    canvas.set_pixel(x, y, [34, 139, 34, 255]); // Forest green floor
                }
            }
            
            // Draw wall
            if draw_start <= draw_end {
                for y in draw_start..=draw_end {
                    if y < CANVAS_HEIGHT {
                        canvas.set_pixel(x, y, wall_color);
                    }
                }
            }
        }
    }
}

fn cast_ray(start: &Vec2f, direction: Vec2f, map: &GameMap) -> Option<RayHit> {
    if direction.x.abs() < 0.00001 && direction.y.abs() < 0.00001 {
        return None; // Invalid direction
    }
    
    let mut map_x = start.x as i32;
    let mut map_y = start.y as i32;
    
    let delta_dist_x = if direction.x.abs() < 0.00001 { 1e30 } else { (1.0 / direction.x).abs() };
    let delta_dist_y = if direction.y.abs() < 0.00001 { 1e30 } else { (1.0 / direction.y).abs() };
    
    let (step_x, mut side_dist_x) = if direction.x < 0.0 {
        (-1, (start.x - map_x as f32) * delta_dist_x)
    } else {
        (1, (map_x as f32 + 1.0 - start.x) * delta_dist_x)
    };
    
    let (step_y, mut side_dist_y) = if direction.y < 0.0 {
        (-1, (start.y - map_y as f32) * delta_dist_y)
    } else {
        (1, (map_y as f32 + 1.0 - start.y) * delta_dist_y)
    };
    
    // DDA (Digital Differential Analyzer)
    let mut side = false;
    
    // Limit iterations to prevent infinite loops
    for _ in 0..100 {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = false;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = true;
        }
        
        // Check bounds
        if map_x < 0 || map_y < 0 || map_x >= map.width as i32 || map_y >= map.height as i32 {
            return None;
        }
        
        let wall_type = map.get_tile(map_x as usize, map_y as usize);
        if wall_type > 0 {
            let perp_wall_dist = if !side {
                (map_x as f32 - start.x + (1 - step_x) as f32 / 2.0) / direction.x
            } else {
                (map_y as f32 - start.y + (1 - step_y) as f32 / 2.0) / direction.y
            };
            
            return Some(RayHit {
                distance: perp_wall_dist.abs().max(0.01),
                wall_type,
                side,
            });
        }
    }
    
    None
}

fn get_wall_color(wall_type: u8, side: bool) -> [u8; 4] {
    let base_color = match wall_type {
        1 => [255, 0, 0],     // Red walls
        2 => [0, 255, 0],     // Green walls
        3 => [0, 0, 255],     // Blue walls
        4 => [255, 255, 0],   // Yellow walls
        5 => [255, 0, 255],   // Magenta walls
        _ => [128, 128, 128], // Default gray
    };
    
    // Make EW walls darker than NS walls for depth perception
    let brightness = if side { 0.7 } else { 1.0 };
    
    [
        (base_color[0] as f32 * brightness) as u8,
        (base_color[1] as f32 * brightness) as u8,
        (base_color[2] as f32 * brightness) as u8,
        255,
    ]
}