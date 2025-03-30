use crate::core::{Color, Maylib};
use sdl2::rect::Rect;

pub struct FontManager {
    pub(crate) ttf: sdl2::ttf::Sdl2TtfContext,
}
impl FontManager {
    pub fn new() -> FontManager {
        FontManager {
            ttf: sdl2::ttf::init().expect("Failed to initialize ttf"),
        }
    }

    pub fn load_font(&mut self, font_path: &str, font_size: u16) -> Font {
        let font = self
            .ttf
            .load_font(font_path, font_size)
            .expect("Failed to load font");
        Font { font }
    }
}
impl Default for FontManager {
    fn default() -> Self {
        FontManager::new()
    }
}

pub struct Font<'a> {
    pub(crate) font: sdl2::ttf::Font<'a, 'static>,
}

impl Maylib {
    pub fn draw_text(&mut self, font: &Font, text: &str, x: i32, y: i32, foreground: Color) {
        let surface = font
            .font
            .render(text)
            .blended(foreground)
            .expect("Failed to draw text");
        let binding = self
            .windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window");
        let texture = binding
            .texture
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture");
        let target = Rect::new(x, y, surface.width(), surface.height());
        binding
            .canvas
            .copy(&texture, None, Some(target))
            .expect("Failed to copy text");
    }

    pub fn measure_text(&self, font: &Font, text: &str) -> (u32, u32) {
        font.font
            .render(text)
            .blended(Color::MayGray)
            .expect("Failed to render text")
            .size()
    }
}
