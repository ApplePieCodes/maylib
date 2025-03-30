use std::collections::HashMap;
use std::rc::Rc;
use sdl2::rect::Rect;
use crate::core::{Color, Maylib};

pub struct FontManager {
    pub(crate) ttf: sdl2::ttf::Sdl2TtfContext,
}
impl FontManager {
    pub fn new() -> FontManager {
        return FontManager {
            ttf: sdl2::ttf::init().unwrap(),
        }
    }

    pub fn load_font(&mut self, font_path: &str, font_name: &str, font_size: u16) -> Font {
        let font = self.ttf.load_font(font_path, font_size).unwrap();
        Font {
            font
        }
    }
}

pub struct Font<'a> {
    pub(crate) font: sdl2::ttf::Font<'a, 'a>,
}

impl Maylib {
    pub fn draw_text(&mut self, font: &Font, text: &str, x: i32, y: i32, foreground: Color) {
        let surface = font.font.render(text).blended(foreground).unwrap();
        let binding = self.windows.get_mut(&self.current_window).unwrap();
        let texture = binding.texture.create_texture_from_surface(&surface).unwrap();
        let target = Rect::new(x, y, surface.width(), surface.height());
        binding.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}