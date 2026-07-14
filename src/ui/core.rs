//! Shared UI colors and primitive drawing helpers.

use macroquad::prelude::*;
use macroquad_toolkit::colors::with_alpha;

pub const BACKGROUND: Color = Color::new(0.008, 0.012, 0.034, 1.0);
pub const PANEL: Color = Color::new(0.015, 0.024, 0.064, 0.93);
pub const PANEL_SOFT: Color = Color::new(0.020, 0.030, 0.078, 0.84);
pub const TEXT_MUTED: Color = Color::new(0.68, 0.73, 0.88, 1.0);
pub const MG_BLUE: Color = Color::new(0.28, 0.78, 1.0, 1.0);
pub const BADDIE_PINK: Color = Color::new(1.0, 0.24, 0.66, 1.0);
pub const PRIORITY_GOLD: Color = Color::new(1.0, 0.78, 0.24, 1.0);
pub const ACCENT_PURPLE: Color = Color::new(0.74, 0.22, 1.0, 1.0);

pub fn draw_panel(x: f32, y: f32, width: f32, height: f32, outline: Color) {
    let rect = Rect::new(x, y, width, height);
    draw_hud_panel(rect, PANEL, outline, true);
}

pub fn draw_soft_panel(x: f32, y: f32, width: f32, height: f32, outline: Color) {
    let rect = Rect::new(x, y, width, height);
    draw_hud_panel(rect, PANEL_SOFT, outline, false);
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

pub fn draw_screen_scrim(alpha: f32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.01, 0.012, 0.032, alpha),
    );
    draw_grid_overlay(alpha * 0.18);
}

pub fn draw_focus_panel(rect: Rect, accent: Color) {
    draw_hud_panel(rect, Color::new(0.014, 0.023, 0.058, 0.90), accent, true);
}

pub fn draw_button_frame(rect: Rect, fill: Color, outline: Color, accent: Color) {
    let cut = chamfer_cut(rect).min(rect.h * 0.26);
    draw_panel_shadow(rect);
    draw_chamfered_rect(rect, cut, fill);
    draw_chamfered_outline(rect, cut, 1.0, with_alpha(outline, 0.72));
    draw_line(
        rect.x + cut,
        rect.y,
        rect.x + rect.w * 0.38,
        rect.y,
        2.0,
        with_alpha(accent, 0.92),
    );
    draw_line(
        rect.x,
        rect.y + cut,
        rect.x,
        rect.y + rect.h - cut,
        4.0,
        with_alpha(accent, 0.82),
    );
}

fn draw_hud_panel(rect: Rect, fill: Color, outline: Color, strong_rule: bool) {
    let cut = chamfer_cut(rect);
    draw_panel_shadow(rect);
    draw_chamfered_rect(rect, cut, fill);
    draw_chamfered_outline(rect, cut, 1.0, with_alpha(outline, 0.46));
    draw_chamfered_outline(
        inset_rect(rect, 8.0),
        (cut - 4.0).max(2.0),
        1.0,
        with_alpha(ACCENT_PURPLE, 0.13),
    );

    let rule_thickness = if strong_rule { 4.0 } else { 2.0 };
    draw_line(
        rect.x + cut,
        rect.y,
        rect.x + rect.w - cut,
        rect.y,
        rule_thickness,
        with_alpha(outline, if strong_rule { 0.86 } else { 0.42 }),
    );
    draw_line(
        rect.x + rect.w - cut * 1.6,
        rect.y,
        rect.x + rect.w - cut * 0.5,
        rect.y + cut * 1.1,
        1.0,
        with_alpha(ACCENT_PURPLE, 0.40),
    );
}

fn draw_panel_shadow(rect: Rect) {
    let shadow_rect = Rect::new(rect.x + 7.0, rect.y + 9.0, rect.w, rect.h);
    draw_chamfered_rect(
        shadow_rect,
        chamfer_cut(rect),
        Color::new(0.0, 0.0, 0.0, 0.32),
    );
}

fn draw_grid_overlay(alpha: f32) {
    let alpha = alpha.clamp(0.0, 0.18);
    if alpha <= 0.0 {
        return;
    }

    let width = screen_width();
    let height = screen_height();
    let step = 34.0;
    let major_step = step * 4.0;
    let minor = Color::new(0.34, 0.48, 0.90, alpha * 0.26);
    let major = Color::new(0.54, 0.20, 0.78, alpha * 0.42);

    let mut x = 0.0;
    while x <= width {
        let color = if (x / major_step).fract() < 0.01 {
            major
        } else {
            minor
        };
        draw_line(x, 0.0, x, height, 1.0, color);
        x += step;
    }

    let mut y = 0.0;
    while y <= height {
        let color = if (y / major_step).fract() < 0.01 {
            major
        } else {
            minor
        };
        draw_line(0.0, y, width, y, 1.0, color);
        y += step;
    }
}

fn draw_chamfered_rect(rect: Rect, cut: f32, color: Color) {
    let points = chamfer_points(rect, cut);
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    for index in 0..points.len() {
        draw_triangle(
            center,
            points[index],
            points[(index + 1) % points.len()],
            color,
        );
    }
}

fn draw_chamfered_outline(rect: Rect, cut: f32, thickness: f32, color: Color) {
    if rect.w <= 0.0 || rect.h <= 0.0 {
        return;
    }
    let points = chamfer_points(rect, cut);
    for index in 0..points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, thickness, color);
    }
}

fn chamfer_points(rect: Rect, cut: f32) -> [Vec2; 8] {
    let cut = cut.min(rect.w * 0.5).min(rect.h * 0.5).max(0.0);
    [
        vec2(rect.x + cut, rect.y),
        vec2(rect.x + rect.w - cut, rect.y),
        vec2(rect.x + rect.w, rect.y + cut),
        vec2(rect.x + rect.w, rect.y + rect.h - cut),
        vec2(rect.x + rect.w - cut, rect.y + rect.h),
        vec2(rect.x + cut, rect.y + rect.h),
        vec2(rect.x, rect.y + rect.h - cut),
        vec2(rect.x, rect.y + cut),
    ]
}

fn chamfer_cut(rect: Rect) -> f32 {
    (rect.w.min(rect.h) * 0.14).clamp(6.0, 24.0)
}

fn inset_rect(rect: Rect, inset: f32) -> Rect {
    Rect::new(
        rect.x + inset,
        rect.y + inset,
        (rect.w - inset * 2.0).max(0.0),
        (rect.h - inset * 2.0).max(0.0),
    )
}
