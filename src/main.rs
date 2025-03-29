use core::{Color, Maylib};

pub mod core;
pub mod image;
mod text;

fn main() {
    let mut may = Maylib::new();
    {
        let window = may.init_window("Froggy", 1280, 720);

        may.load_font("C:\\Users\\Liam Greenway\\maylib\\font.ttf", "mayfont", 20);

        may.switch_window(window);

        while !may.window_should_close() {
            may.begin_drawing();

            may.clear_background(Color::MayGray);

            may.draw_image("C:\\Users\\Liam Greenway\\maylib\\Frog-tree.jpg", 50, 50);
            may.draw_text("mayfont", "Hello Freg", 100, 100, Color::White);

            may.end_drawing()
        }

        may.close_window();
    }
}