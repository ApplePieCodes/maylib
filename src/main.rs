use crate::text::FontManager;
use core::{Color, Maylib};
use sdl2::keyboard::Scancode;

pub mod core;
pub mod image;
mod shapes;
mod text;

fn main() {
    let mut may = Maylib::new().expect("Code in main should be correct");
    let mut fontmgr = FontManager::new();
    let font = fontmgr.load_font(
        "C:\\Users\\Liam Greenway\\RustroverProjects\\maylib\\font.ttf",
        20,
    );
    {
        let window = may
            .init_window("Froggy", 1280, 720)
            .expect("Code in main should be correct");

        may.switch_window(window);

        let mut x = 0;
        let mut y = 0;

        while !may.window_should_close() {
            may.begin_drawing();

            may.clear_background(Color::MayGray);

            if may.key_pressed(Scancode::W) {
                y -= 1;
            }
            if may.key_pressed(Scancode::S) {
                y += 1;
            }
            if may.key_pressed(Scancode::A) {
                x -= 1;
            }
            if may.key_pressed(Scancode::D) {
                x += 1;
            }

            may.draw_rectangle(x, y, 50, 50, Color::RayWhite);

            may.end_drawing()
        }

        may.close_window();
    }
}
