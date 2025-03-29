use sdl2::image::LoadSurface;
use crate::core::Maylib;

impl Maylib {
    pub fn set_window_icon(&mut self, path: &str) {
        self.windows.get_mut(&self.current_window).unwrap().window.set_icon(sdl2::surface::Surface::from_file(path).unwrap());
    }
}