pub mod renderer;
pub use raylib::prelude::RaylibDraw;
pub mod audio;
pub mod window;
#[test]
fn test() {
    use raylib::prelude::Vector2;
    let mut win = crate::window::Window::init("cool",Vector2 { x: 300., y: 300. },60 );
    win.mainloop(|mut r| {
        r.set_background(10, 10, 10);
    });
}
#[cfg(test)]
mod audio_test {
    #[test]
    fn audio_playback() {
        let mut audio = crate::audio::AudioChannel::new().unwrap();
        audio.add_audio("bruh", "test.mp3");
        let sender = audio.get_sender();
        audio.audio_listener();
        sender.send("bruh").unwrap();
        // let the music play for a bit
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
