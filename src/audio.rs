use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use crate::core::MAYLIB;

pub fn play_sound(path: &str) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    // Load the audio
    let file = BufReader::new(File::open(path).expect("File not found"));
    // Decode
    let source = Decoder::new(file).expect("File not valid");
    // And play. It's that shrimple
    if let Some(a) = get.audio.as_mut() {
        a
            .play_raw(source.convert_samples())
            .expect("TODO: panic message");
    }
}