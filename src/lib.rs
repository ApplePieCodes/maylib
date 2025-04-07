pub mod audio;
pub mod core;
pub mod image;
pub mod shapes;
pub mod text;
pub mod types;

#[cfg(test)]
pub mod test {
    use std::os::windows;
    use crate::audio::play_sound;
    use crate::core::{begin_drawing, clear_background, end_drawing, init_window, switch_window, window_should_close};
    use crate::text::{draw_text, load_font_bytes};
    use crate::types::{Color, Maylib};

    #[test]
    pub fn text_test() {
        let window = init_window("Text Test", 640, 480)
            .expect("Code in tests should be correct");
        let fira = load_font_bytes(include_bytes!(".././fira-code.ttf"));
        switch_window(window);
        while !window_should_close() {
            begin_drawing();

            clear_background(Color::MayGray);
            draw_text(&fira, "Hello Maylib", 32.0, 50, 50, Color::Lime);

            end_drawing();
        }
    }

    #[test]
    pub fn audio_test() {
        let window = init_window("Audio Test", 500, 500)
            .expect("Code in tests should be correct");
        switch_window(window);
        play_sound("./test-beep.mp3");
        while !window_should_close() {
            begin_drawing();

            clear_background(Color::MayGray);

            end_drawing();
        }
    }
}
