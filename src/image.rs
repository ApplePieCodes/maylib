use crate::core::Maylib;
use sdl2::image::{LoadSurface, LoadTexture};

impl Maylib {
    pub fn set_window_icon(&mut self, path: &str) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .set_icon(sdl2::surface::Surface::from_file(path).expect("Can't load image"));
    }

    pub fn draw_image(&mut self, path: &str, x: i32, y: i32) {
        let window = self
            .windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window");
        let texture = window.texture.load_texture(path).expect("Can't load image");
        let texture_query = texture.query();
        // Create destination rectangle at (x, y) with the texture's width and height
        let dst_rect = sdl2::rect::Rect::new(x, y, texture_query.width, texture_query.height);
        window
            .canvas
            .copy(&texture, None, Some(dst_rect))
            .expect("Can't copy image");
    }

    // TODO: More image operations, ie. image_resize, image_stretch, dither, etc
}
