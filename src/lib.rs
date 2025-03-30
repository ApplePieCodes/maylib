pub mod core;
pub mod image;
pub mod shapes;
pub mod text;
mod audio;

#[cfg(test)]
pub mod test {
    use crate::core::{Color, Maylib};

    #[test]
    pub fn audio_test() {
        let mut maylib = Maylib::new().expect("Code in tests should be correct");
        let window = maylib.init_window("", 500, 500).expect("Code in tests should be correct");
        maylib.switch_window(window);
        maylib.play_sound("./baldi_testaudio.wav");
        while !maylib.window_should_close() {
            maylib.begin_drawing();

            maylib.clear_background(Color::MayGray);

            maylib.end_drawing();
        }
    }
}