use crate::core::Maylib;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;

impl Maylib {
    /// Plays the audio file at path
    pub fn play_sound(&mut self, path: &str) {
        // Load the audio
        let file = BufReader::new(File::open(path).expect("File not found"));
        // Decode
        let source = Decoder::new(file).expect("File not valid");
        // And play. It's that shrimple
        self.audio.play_raw(source.convert_samples()).expect("TODO: panic message");
    }
}