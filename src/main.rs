use core::{Color, Maylib};

mod core;
mod image;

fn main() {
    let mut may = Maylib::new();
    let win = may.init_window(640, 480, "Hello Everynyan");
    let win2 = may.init_window(640, 480, "Hello Somenyan");

    may.switch_window(win);

    while !may.all_windows_closed() {
        may.switch_window(win);
        if may.window_should_close() {
            may.close_window();
        }
        else {
            may.begin_drawing();
            may.clear_background(Color::new(255, 255, 255));
            may.end_drawing();
        }
        may.switch_window(win2);
        if may.window_should_close() {
            may.close_window();
        }
        else {
            may.begin_drawing();
            may.clear_background(Color::new(255, 255, 255));
            may.end_drawing();
        }
    }
}