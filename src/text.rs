use crate::types::{Color};
use fontdue_sdl2::fontdue::FontSettings;
use fontdue_sdl2::fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue_sdl2::{FontTexture, fontdue};
use sdl2::pixels;
use std::fs::File;
use std::io::Read;
use sdl2::render::Canvas;
use sdl2::sys::Window;
use crate::core::MAYLIB;

pub struct Font {
    font: fontdue::Font,
}

pub fn load_font_file(path: &str) -> Font {
    let mut buffer: Vec<u8> = vec![];
    let mut file = File::open(path).expect("Failed to load font");
    file.read_to_end(&mut buffer).expect("TODO: panic message");
    Font {
        font: fontdue::Font::from_bytes(buffer, FontSettings::default())
            .expect("Failed to load font"),
    }
}

pub fn load_font_bytes(bytes: &[u8]) -> Font {
    Font {
        font: fontdue::Font::from_bytes(bytes, FontSettings::default())
            .expect("Failed to load font"),
    }
}

pub fn draw_text(font: &Font, text: &str, size: f32, x: i32, y: i32, color: Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    let fonts = [font.font.clone()];

    layout.reset(&fontdue::layout::LayoutSettings {
        x: x as f32, // Apply X offset
        y: y as f32, // Apply Y offset
        ..Default::default()
    });

    layout.append(
        &fonts,
        &TextStyle::with_user_data(text, size, 0, pixels::Color::from(color)),
    );

    if let Some(window) = get.windows.get_mut(&current_window) {
        let mut font_texture = FontTexture::new(&window.texture).expect("Failed to load font");
        let _ = font_texture.draw_text(&mut window.canvas, &fonts, layout.glyphs().as_slice());
    }
}

pub fn measure_text_width(font: &Font, text: &str, size: f32) -> f32 {
    let mut width = 0.0;
    for c in text.chars() {
        let (metrics, _) = font.font.rasterize(c, size);
        width += metrics.advance_width;
    }
    
    width
}

pub fn measure_text_height(font: &Font, text: &str, size: f32) -> f32 {
    if text.is_empty() {
        0.0
    }
    else {
        let (metrics, _) = font.font.rasterize(text.chars().next().expect("Should have a [0] because it's not empty"), size);
        metrics.height as f32
    }
}