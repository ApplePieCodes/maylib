use fontdue_sdl2::fontdue::{Font, FontSettings};
use fontdue_sdl2::fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::pixels;
use crate::core::{Color, Maylib};

impl Maylib {
    pub fn load_font(&mut self, bytes: &[u8]) -> Font {
        Font::from_bytes(bytes, FontSettings::default()).expect("Failed to load font")
    }

    pub fn draw_text(&mut self, font: &Font, text: &str, size: f32, x: i32, y: i32, color: Color) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        let fonts = [font.clone()];

        layout.reset(&fontdue_sdl2::fontdue::layout::LayoutSettings {
            x: x as f32,  // Apply X offset
            y: y as f32,  // Apply Y offset
            ..Default::default()
        });

        layout.append(
            &fonts,
            &TextStyle::with_user_data(
                text,
                size,
                0,
                pixels::Color::from(color),
            )
        );

        if let Some(window) = self.windows.get_mut(&self.current_window) {
            let mut font_texture = FontTexture::new(&window.texture).expect("Failed to load font");
            let _ = font_texture.draw_text(&mut window.canvas, &fonts, layout.glyphs().as_slice());
        }
    }

}
