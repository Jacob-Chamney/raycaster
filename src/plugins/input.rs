use bevy::prelude::*;
use super::canvas::PixelCanvas;
use super::player::Player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_canvas_controls,
            handle_debug_controls,
        ));
    }
}

fn handle_canvas_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut canvas: ResMut<PixelCanvas>,
    player: Option<Res<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Drawing random pixels");
        draw_random_pixels(&mut canvas);
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        info!("Clearing canvas");
        canvas.clear([0, 0, 40, 255]);
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        info!("Drawing red pattern");
        draw_red_pattern(&mut canvas);
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        info!("Drawing green pattern");
        draw_green_pattern(&mut canvas);
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        info!("Drawing blue pattern");
        draw_blue_pattern(&mut canvas);
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        if let Some(player) = player {
            info!("Player pos: ({:.2}, {:.2}), angle: {:.2} rad ({:.1}°)", 
                  player.position.x, player.position.y, 
                  player.angle, player.angle * 180.0 / std::f32::consts::PI);
        }
    }
}

fn handle_debug_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Exit requested");
        exit.write(AppExit::Success);
    }
    
    if keyboard_input.just_pressed(KeyCode::F1) {
        info!("Controls: [WASD] Move, [Mouse] Look, [P] Player info, [M] Toggle minimap, [F1] Help, [Esc] Exit/Release mouse");
    }
}

fn draw_random_pixels(canvas: &mut PixelCanvas) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;
    
    for i in 0..10 {
        let x = time.wrapping_add(i * 37) % canvas.width;
        let y = time.wrapping_add(i * 73) % canvas.height;
        let color = [
            ((time.wrapping_add(i * 17)) % 256) as u8,
            ((time.wrapping_add(i * 31)) % 256) as u8,
            ((time.wrapping_add(i * 43)) % 256) as u8,
            255,
        ];
        canvas.set_pixel(x, y, color);
    }
}

fn draw_red_pattern(canvas: &mut PixelCanvas) {
    let x = (canvas.width / 4) as u32;
    let y = (canvas.height / 4) as u32;
    canvas.draw_rect(x, y, 50, 50, [255, 100, 100, 255]);
}

fn draw_green_pattern(canvas: &mut PixelCanvas) {
    let x = (canvas.width / 2) as u32;
    let y = (canvas.height / 4) as u32;
    canvas.draw_rect(x, y, 40, 60, [100, 255, 100, 255]);
}

fn draw_blue_pattern(canvas: &mut PixelCanvas) {
    let x = (canvas.width * 3 / 4) as u32;
    let y = (canvas.height / 4) as u32;
    canvas.draw_rect(x, y, 30, 70, [100, 100, 255, 255]);
}