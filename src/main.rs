use core::Maylib;

mod core;

fn main() {
    let mut may = Maylib::new();
    let win = may.init_window("Hello Everynyan", 640, 480);
    let win2 = may.init_window("Hello Somenyan", 640, 480);

    while !may.all_windows_closed() {
        may.handle_events();
        may.switch_window(win);
        if may.should_close() {
            may.close_window();
        }
        else {
            may.clear();
            may.draw();
        }
        may.switch_window(win2);
        if may.should_close() {
            may.close_window();
        }
        else {
            may.clear();
            may.draw();
        }
    }
}