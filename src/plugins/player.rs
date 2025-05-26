use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use super::math::{Vec2f, normalize_angle};
use super::map::GameMap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, (
                handle_mouse_capture,
                handle_mouse_look,
                handle_movement,
                update_player_direction,
            ));
    }
}

#[derive(Resource)]
pub struct Player {
    pub position: Vec2f,
    pub angle: f32,      // Horizontal angle (yaw)
    pub pitch: f32,      // Vertical angle (pitch)
    pub direction: Vec2f,
    pub plane: Vec2f,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        let angle = 0.0;
        Self {
            position: Vec2f::new(12.0, 12.0),
            angle,
            pitch: 0.0, // Start looking straight ahead
            direction: Vec2f::from_angle(angle),
            plane: Vec2f::new(0.0, 0.66),
            move_speed: 3.0,
            rotation_speed: 3.0,
            mouse_sensitivity: 0.003,
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands.insert_resource(Player::default());
    info!("Player initialized at position (12.0, 12.0) - center of map");
    info!("Click window to capture mouse for FPS controls");
}

fn handle_mouse_capture(
    mut windows: Query<&mut Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if mouse_button.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            info!("Mouse captured - WASD to move, mouse to look, ESC to release");
        }
        
        if keyboard_input.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            info!("Mouse released - click to recapture");
        }
    }
}

fn handle_mouse_look(
    mut player: ResMut<Player>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for event in mouse_motion.read() {
        let delta_x = event.delta.x;
        let delta_y = event.delta.y;
        
        // Horizontal rotation (yaw)
        player.angle += delta_x * player.mouse_sensitivity;
        player.angle = normalize_angle(player.angle);
        
        // Vertical rotation (pitch) - clamp to prevent over-rotation
        player.pitch -= delta_y * player.mouse_sensitivity;
        player.pitch = player.pitch.clamp(-4.0, 4.0); // Limit pitch to prevent extreme values
        
        // Update direction vectors
        player.direction = Vec2f::from_angle(player.angle);
        player.plane = Vec2f::from_angle(player.angle + std::f32::consts::PI / 2.0) * 0.66;
    }
}

fn handle_movement(
    mut player: ResMut<Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Option<Res<GameMap>>,
) {
    let move_speed = player.move_speed * time.delta_secs();
    let mut new_position = player.position;
    
    if keyboard_input.pressed(KeyCode::KeyW) {
        let next_pos = player.position + player.direction * move_speed;
        if let Some(ref map) = map {
            if map.is_valid_position(next_pos) {
                new_position = next_pos;
            }
        } else {
            new_position = next_pos;
        }
    }
    
    if keyboard_input.pressed(KeyCode::KeyS) {
        let next_pos = player.position - player.direction * move_speed;
        if let Some(ref map) = map {
            if map.is_valid_position(next_pos) {
                new_position = next_pos;
            }
        } else {
            new_position = next_pos;
        }
    }
    
    if keyboard_input.pressed(KeyCode::KeyA) {
        let left_dir = player.direction.rotate(-std::f32::consts::PI / 2.0);
        let next_pos = player.position + left_dir * move_speed;
        if let Some(ref map) = map {
            if map.is_valid_position(next_pos) {
                new_position = next_pos;
            }
        } else {
            new_position = next_pos;
        }
    }
    
    if keyboard_input.pressed(KeyCode::KeyD) {
        let right_dir = player.direction.rotate(std::f32::consts::PI / 2.0);
        let next_pos = player.position + right_dir * move_speed;
        if let Some(ref map) = map {
            if map.is_valid_position(next_pos) {
                new_position = next_pos;
            }
        } else {
            new_position = next_pos;
        }
    }
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player.angle -= player.rotation_speed * time.delta_secs();
        player.angle = normalize_angle(player.angle);
    }
    
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player.angle += player.rotation_speed * time.delta_secs();
        player.angle = normalize_angle(player.angle);
    }
    
    if (new_position.x - player.position.x).abs() > 0.001 || 
       (new_position.y - player.position.y).abs() > 0.001 {
        player.position = new_position;
    }
}

fn update_player_direction(
    mut player: ResMut<Player>,
) {
    if player.is_changed() {
        player.direction = Vec2f::from_angle(player.angle);
        player.plane = Vec2f::from_angle(player.angle + std::f32::consts::PI / 2.0) * 0.66;
    }
}