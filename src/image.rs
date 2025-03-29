use sdl2::image::{LoadSurface, LoadTexture};
use crate::core::Maylib;

impl<'a> Maylib<'a> {
    pub fn set_window_icon(&mut self, path: &str) {
        self.windows.get_mut(&self.current_window).unwrap().window.set_icon(sdl2::surface::Surface::from_file(path).unwrap());
    }

    pub fn draw_image(&mut self, path: &str, x: i32, y: i32) {
        let window = self.windows.get_mut(&self.current_window).unwrap();
        let texture = window.texture.load_texture(path).unwrap();
        let texture_query = texture.query();
        // Create destination rectangle at (x, y) with the texture's width and height
        let dst_rect = sdl2::rect::Rect::new(x, y, texture_query.width, texture_query.height);
        window.canvas.copy(&texture, None, Some(dst_rect)).unwrap();
    }

    // TODO: More image operations, ie. image_resize, image_stretch, dither, etc
}