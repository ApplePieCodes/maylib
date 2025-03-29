use std::rc::Rc;
use sdl2::rect::Rect;
use crate::core::{Color, Maylib};

impl Maylib {
    pub fn load_font(&mut self, font_path: &str, font_name: &str, font_size: u16) {
        let font = self.ttf.load_font(font_path, font_size).unwrap();
        self.fonts.insert(font_name.to_string(), Rc::new(font));
    }

    pub fn draw_text(&mut self, font: &str, text: &str, x: i32, y: i32, foreground: Color) {
        let font = self.fonts.get_mut(font).unwrap();
        let surface = font.render(text).blended(foreground).unwrap();
        let binding = self.windows.get_mut(&self.current_window).unwrap();
        let texture = binding.texture.create_texture_from_surface(&surface).unwrap();
        let target = Rect::new(x, y, surface.width(), surface.height());
        binding.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}