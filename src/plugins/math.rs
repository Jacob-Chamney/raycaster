use bevy::prelude::*;
use std::f32::consts::PI;

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MathConstants::default());
    }
}

#[derive(Resource)]
pub struct MathConstants {
    pub two_pi: f32,
    pub half_pi: f32,
    pub pi: f32,
    pub deg_to_rad: f32,
    pub rad_to_deg: f32,
}

impl Default for MathConstants {
    fn default() -> Self {
        Self {
            two_pi: 2.0 * PI,
            half_pi: PI / 2.0,
            pi: PI,
            deg_to_rad: PI / 180.0,
            rad_to_deg: 180.0 / PI,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn _normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }
    
    pub fn _dot(&self, other: &Vec2f) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn rotate(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }
    
    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    
    pub fn _to_angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl std::ops::Add for Vec2f {
    type Output = Vec2f;
    
    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2f {
    type Output = Vec2f;
    
    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2f {
    type Output = Vec2f;
    
    fn mul(self, scalar: f32) -> Vec2f {
        Vec2f {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut normalized = angle % (2.0 * PI);
    if normalized < 0.0 {
        normalized += 2.0 * PI;
    }
    normalized
}

pub fn _lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn _clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}