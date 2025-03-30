use crate::core::Maylib;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;

impl Maylib {
    pub fn play_sound(&mut self, path: &str) {
        let file = BufReader::new(File::open(path).expect("File not found"));
        // Decode that sound file into a source
        let source = Decoder::new(file).expect("File not valid");
        // Play the sound directly on the device
        self.audio.play_raw(source.convert_samples()).expect("TODO: panic message");
    }
}