//! Shared 1440p-first layout helpers.

use macroquad::prelude::{screen_height, screen_width, Rect};

pub const BASE_WIDTH: f32 = 2560.0;
pub const BASE_HEIGHT: f32 = 1440.0;

#[derive(Clone, Copy, Debug)]
pub struct UiLayout {
    scale_x: f32,
    scale_y: f32,
}

impl UiLayout {
    pub fn current() -> Self {
        Self {
            scale_x: screen_width() / BASE_WIDTH,
            scale_y: screen_height() / BASE_HEIGHT,
        }
    }

    pub fn x(&self, value: f32) -> f32 {
        value * self.scale_x
    }

    pub fn y(&self, value: f32) -> f32 {
        value * self.scale_y
    }

    pub fn w(&self, value: f32) -> f32 {
        value * self.scale_x
    }

    pub fn h(&self, value: f32) -> f32 {
        value * self.scale_y
    }

    pub fn font(&self, value: f32) -> f32 {
        value * self.scale_y.min(self.scale_x)
    }

    pub fn rect(&self, x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect::new(self.x(x), self.y(y), self.w(width), self.h(height))
    }
}
