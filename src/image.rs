use crate::core::MAYLIB;
use sdl2::image::{LoadSurface, LoadTexture};

pub fn set_window_icon(path: &str) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .set_icon(sdl2::surface::Surface::from_file(path).expect("Can't load image"));
}

pub fn draw_image(path: &str, x: i32, y: i32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get
        .windows
        .get_mut(&current_window)
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

