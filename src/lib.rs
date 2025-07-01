// use std::time::Instant;
use ::rand::*;
pub use macroquad::prelude::*;

#[derive(Clone)]
pub struct Car {
    pub dir: KeyCode,
    pub pos: (f32, f32),
    pub color: Color,
}

impl Car {
    pub fn new(dir: KeyCode, pos: (f32, f32)) -> Self {
        let color = match random_range(0..3) {
            0 => RED,
            1 => YELLOW,
            _ => GREEN,
        };
        Self { dir, pos, color }
    }
    pub fn random_color() -> Color {
        match random_range(0..3) {
            0 => RED,
            1 => YELLOW,
            _ => GREEN,
        }
    }
}
