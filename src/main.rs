use core::{Color, Maylib};
use crate::text::FontManager;

pub mod core;
pub mod image;
mod text;
mod shapes;

fn main() {
    let mut may = Maylib::new();
    let mut fontmgr = FontManager::new();
    let font = fontmgr.load_font("C:\\Users\\Liam Greenway\\RustroverProjects\\maylib\\font.ttf", 20);
    {
        let window = may.init_window("Froggy", 1280, 720);

        may.switch_window(window);

        while !may.window_should_close() {
            may.begin_drawing();

            may.clear_background(Color::MayGray);

            may.draw_image("C:\\Users\\Liam Greenway\\RustroverProjects\\maylib\\Frog-tree.jpg", 50, 50);
            let tsize = may.measure_text(&font, "Hello Freg");
            let size = may.get_window_size();
            let x = size.0 / 2 - tsize.0 / 2;
            let y = size.1 / 2 - tsize.1 / 2;
            may.draw_text(&font, "Hello Freg", x as i32, y as i32, Color::Red);
            may.draw_circle(500, 500, 500, Color::Green);

            may.end_drawing()
        }

        may.close_window();
    }
}