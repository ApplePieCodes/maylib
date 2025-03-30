use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::rect::{Point, Rect};
use crate::core::{Color, Maylib};

impl Maylib {
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(color);
        self.windows.get_mut(&self.current_window).unwrap().canvas.draw_point(Point::new(x, y));
    }

    pub fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(color);
        self.windows.get_mut(&self.current_window).unwrap().canvas.draw_line(Point::new(start_x, start_y), Point::new(end_x, end_y));
    }

    pub fn draw_circle(&mut self, center_x: i16, center_y: i16, radius: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.filled_circle(center_x, center_y, radius, pixels::Color::from(color)).unwrap()
    }

    pub fn draw_circle_lines(&mut self, center_x: i16, center_y: i16, radius: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.circle(center_x, center_y, radius, pixels::Color::from(color)).unwrap()
    }

    pub fn draw_ellipse(&mut self, x: i16, y: i16, w: i16, h: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.filled_ellipse(x, y, w, h, pixels::Color::from(color)).unwrap();
    }

    pub fn draw_ellipse_lines(&mut self, x: i16, y: i16, w: i16, h: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.ellipse(x, y, w, h, pixels::Color::from(color)).unwrap();
    }

    pub fn draw_rectangle(&mut self, x: i16, y: i16, w: i16, h: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(color);
        let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);
        self.windows.get_mut(&self.current_window).unwrap().canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_rectangle_lines(&mut self, x: i16, y: i16, w: i16, h: i16, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.rectangle(x, y, w, h, pixels::Color::from(color)).unwrap();
    }
}