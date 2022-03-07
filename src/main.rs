use tts::{Backends, Tts};

fn main() {
    let mut tts = Tts::new(Backends::SpeechDispatcher).unwrap();

    let tts_clone = tts.clone();

    tts.speak("Hello!", true).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2))
}
