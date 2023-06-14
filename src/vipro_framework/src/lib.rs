pub mod renderer;
pub use raylib::prelude::{RaylibDraw,Color as RColor};
pub mod audio;
pub mod window;
pub mod update;
pub mod graphics2d;
pub mod graphics3d;
#[derive(Debug, Clone, Copy,PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Color {
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:Option<u8>,
}
impl Color {
    pub fn to_color(&self) -> RColor {
        RColor::new(self.r, self.g, self.b, match self.a {Some(o) => o, None => 255 })
    }
}
#[test]
fn window_test() {
    use raylib::prelude::Vector2;
    let mut win = crate::window::Window::init("cool",Vector2 { x: 300., y: 300. },60 );
    win.mainloop(|mut r| {
        r.set_background(10, 10, 10);
    });
}
#[test]
fn rectangle_test() {
    use crate::window::VVector2;
    use crate::graphics2d::*;
    let mut win = crate::window::Window::init("cool",VVector2 {x:300.,y:300.} , 60);
    let rect = crate::graphics2d::Rectangle::new(Vector2d { x: 100., y: 100. }, Vector2d { x: 100., y: 50. }, Color { r: 255, g: 120, b: 0, a: None });
    let poly = crate::graphics2d::Polygon::new(Vector2d { x: 100., y: 30. }, 100., 3, 0.0, Color { r: 100, g: 100, b: 30, a: None });
    let lined_circle =LinedCircle::new(Vector2d { x: 100., y: 100. },100.,10,Color { r: 255, g: 0, b: 255, a: Some(50) });
    win.mainloop(|mut ren| {
        ren.set_background(100, 200, 50);
        ren.render_object(&rect);
        ren.render_object(&poly);
        ren.render_object(&lined_circle)
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
        sender.play_sound("bruh").unwrap();
        // let the music play for a bit
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
