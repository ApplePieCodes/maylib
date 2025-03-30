use sdl2::rect::Point;
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

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 1 - x;

        while x >= y {
            self.draw_scanline(center_x, center_y, x, y, color.clone());
            self.draw_scanline(center_x, center_y, y, x, color.clone());

            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }

    fn draw_scanline(&mut self, cx: i32, cy: i32, x: i32, y: i32, color: Color) {
        let mut canvas = &mut self.windows.get_mut(&self.current_window.clone()).unwrap().canvas;
        canvas.set_draw_color(color);
        canvas.draw_line(Point::new(cx - x, cy + y), Point::new(cx + x, cy + y)).unwrap();
        canvas.draw_line(Point::new(cx - x, cy - y), Point::new(cx + x, cy - y)).unwrap();
    }

    pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(color);
        let mut canvas = &mut self.windows.get_mut(&self.current_window.clone()).unwrap().canvas;
        let mut x = radius;
        let mut y = 0;
        let mut err = 1 - x;

        while x >= y {
            let points = [
                (center_x + x, center_y + y),
                (center_x + y, center_y + x),
                (center_x - y, center_y + x),
                (center_x - x, center_y + y),
                (center_x - x, center_y - y),
                (center_x - y, center_y - x),
                (center_x + y, center_y - x),
                (center_x + x, center_y - y),
            ];

            for (px, py) in points {
                canvas.draw_point(Point::new(px, py)).unwrap();
            }

            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }
}