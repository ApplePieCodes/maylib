use crate::types::{Color};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::rect::{Point, Rect};
use crate::core::MAYLIB;

pub fn draw_pixel(x: i32, y: i32, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .set_draw_color(color);
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .draw_point(Point::new(x, y))
        .expect("Failed to draw pixel");
}

pub fn draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .set_draw_color(color);
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .draw_line(Point::new(start_x, start_y), Point::new(end_x, end_y))
        .expect("Failed to draw line");
}

pub fn draw_circle(center_x: i16, center_y: i16, radius: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .filled_circle(center_x, center_y, radius, pixels::Color::from(color))
        .expect("Failed to fill circle");
}

pub fn draw_circle_lines(center_x: i16, center_y: i16, radius: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .circle(center_x, center_y, radius, pixels::Color::from(color))
        .expect("Failed to draw circle");
}

pub fn draw_ellipse(x: i16, y: i16, w: i16, h: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .filled_ellipse(x, y, w, h, pixels::Color::from(color))
        .expect("Failed to fill ellipse");
}

pub fn draw_ellipse_lines(x: i16, y: i16, w: i16, h: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .ellipse(x, y, w, h, pixels::Color::from(color))
        .expect("Failed to draw ellipse");
}

pub fn draw_rectangle(x: i16, y: i16, w: i16, h: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .set_draw_color(color);
    let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .fill_rect(rect)
        .expect("Failed to fill rectangle");
}

pub fn draw_rectangle_lines(x: i16, y: i16, w: i16, h: i16, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .rectangle(x, y, w, h, pixels::Color::from(color))
        .expect("Failed to draw rectangle");
}