use std::cell::RefCell;

// use std::time::Instant;
use ::rand::*;
pub use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Car {
    pub dir: KeyCode,
    pub pos: (f32, f32),
    pub color: Color,
    pub is_moved: bool,
    pub stop: bool,
    pub in_intersection: bool,
}

pub struct Lights {
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
        Self {
            dir,
            pos,
            color,
            is_moved: false,
            stop: false,
            in_intersection: false,
        }
    }
    pub fn random_color() -> Color {
        match random_range(0..3) {
            0 => RED,    // direct
            1 => YELLOW, // Left
            _ => GREEN,  // Right
        }
    }
}
