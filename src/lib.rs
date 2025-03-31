pub mod core;
pub mod image;
pub mod shapes;
pub mod text;
mod audio;
mod text_new;

#[cfg(test)]
pub mod test {
    use crate::core::{Color, Maylib};

    #[test]
    pub fn text_test() {
        let mut maylib = Maylib::init().expect("Code in tests should be correct");
        let window = maylib.init_window("Text Test", 640, 480).expect("Code in tests should be correct");
        let fira = maylib.load_font_bytes(include_bytes!(".././fira-code.ttf"));
        maylib.switch_window(window);
        while !maylib.window_should_close() {
            maylib.begin_drawing();

            maylib.clear_background(Color::MayGray);
            maylib.draw_text(&fira, "Hello Maylib", 32.0, 50, 50, Color::Lime);

            maylib.end_drawing();
        }
    }

    #[test]
    pub fn audio_test() {
        let mut maylib = Maylib::init().expect("Code in tests should be correct");
        let window = maylib.init_window("Audio Test", 500, 500).expect("Code in tests should be correct");
        maylib.switch_window(window);
        maylib.play_sound("./baldi_testaudio.wav");
        while !maylib.window_should_close() {
            maylib.begin_drawing();

            maylib.clear_background(Color::MayGray);

            maylib.end_drawing();
        }
    }
}