use std::time::Duration;

use vipro_framework::*;
fn main() {
    let mut a = audio::AudioChannel::new().unwrap();
    a.add_audio("test", "test.wav").unwrap();
    let s = a.get_sender();
    a.audio_listener();
    s.send("test").unwrap();
    std::thread::sleep(Duration::from_secs(10));
}
