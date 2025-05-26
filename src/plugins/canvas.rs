use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const CANVAS_WIDTH: u32 = 400;
pub const CANVAS_HEIGHT: u32 = 300;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_canvas)
            .add_systems(Update, update_canvas_display);
    }
}

#[derive(Resource)]
pub struct PixelCanvas {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl PixelCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height * 4) as usize;
        info!("Creating canvas: {}x{} = {} pixels = {} bytes", 
              width, height, width * height, pixel_count);
        
        Self {
            pixels: vec![0; pixel_count],
            width,
            height,
        }
    }
    
    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let index = ((y * self.width + x) * 4) as usize;
        self.pixels[index..index + 4].copy_from_slice(&color);
    }
    
    pub fn clear(&mut self, color: [u8; 4]) {
        for chunk in self.pixels.chunks_mut(4) {
            chunk.copy_from_slice(&color);
        }
    }
    
    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }
}

#[derive(Component)]
pub struct CanvasSprite;

fn setup_canvas(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut canvas = PixelCanvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    
    canvas.clear([0, 0, 40, 255]);
    canvas.draw_rect(50, 50, 100, 80, [255, 0, 0, 255]);
    canvas.draw_rect(200, 100, 60, 60, [0, 255, 0, 255]);
    canvas.set_pixel(10, 10, [255, 255, 255, 255]);
    
    let image = Image::new(
        Extent3d {
            width: canvas.width,
            height: canvas.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        canvas.pixels.clone(),
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );
    
    let image_handle = images.add(image);
    
    commands.spawn((
        Sprite {
            image: image_handle,
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0)),
        CanvasSprite,
    ));
    
    commands.insert_resource(canvas);
    info!("Pixel canvas initialized with test patterns");
}

fn update_canvas_display(
    mut images: ResMut<Assets<Image>>,
    canvas: Res<PixelCanvas>,
    query: Query<&Sprite, With<CanvasSprite>>,
) {
    if canvas.is_changed() {
        for sprite in query.iter() {
            if let Some(image) = images.get_mut(&sprite.image) {
                image.data = Some(canvas.pixels.clone());
            }
        }
    }
}