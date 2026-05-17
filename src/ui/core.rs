//! Shared UI colors and primitive drawing helpers.

use macroquad::prelude::*;

pub const BACKGROUND: Color = Color::new(0.09, 0.08, 0.14, 1.0);
pub const PANEL: Color = Color::new(0.12, 0.12, 0.20, 0.96);
pub const PANEL_SOFT: Color = Color::new(0.16, 0.16, 0.26, 0.92);
pub const TEXT_MUTED: Color = Color::new(0.76, 0.78, 0.86, 1.0);

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32, outline: Color) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL).with_border(2.0, outline);
    macroquad_toolkit::ui::draw_surface(Rect::new(x, y, width, height), &surface);
}

pub fn draw_soft_panel(x: f32, y: f32, width: f32, height: f32, outline: Color) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL_SOFT).with_border(2.0, outline);
    macroquad_toolkit::ui::draw_surface(Rect::new(x, y, width, height), &surface);
}

pub fn draw_background_texture(texture: &Texture2D, tint: Color) {
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
}
