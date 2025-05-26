use bevy::prelude::*;
use super::math::Vec2f;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

#[derive(Resource)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<u8>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![0; width]; height];
        Self { width, height, tiles }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            return 1; // Wall if out of bounds
        }
        self.tiles[y][x]
    }
    
    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;
        self.get_tile(map_x, map_y) != 0
    }
    
    pub fn is_valid_position(&self, pos: Vec2f) -> bool {
        if pos.x < 0.0 || pos.y < 0.0 {
            return false;
        }
        !self.is_wall(pos.x, pos.y)
    }
}

fn setup_map(mut commands: Commands) {
    let mut map = GameMap::new(24, 24);
    
    // Create a classic maze-like level
    let level_data = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,4,4,4,4,4,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];
    
    for (y, row) in level_data.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            map.tiles[y][x] = tile;
        }
    }
    
    commands.insert_resource(map);
    info!("Game map loaded: 24x24 with walls and obstacles");
}