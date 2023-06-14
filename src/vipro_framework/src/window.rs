use raylib::prelude::*;
use crate::renderer;
#[derive(Debug)]
pub struct Window {
    handle:RaylibHandle,
    thread:RaylibThread
}
#[derive(Debug,Clone)]
pub struct VVector2 {
    pub x:f32,
    pub y:f32
}
impl Into<Vector2> for VVector2 {
    fn into(self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
}
impl Window {
    /// Creates a Window so it can later be used to render objects to the screen
    pub fn init(title:impl AsRef<str>,dimensions:impl Into<Vector2>,fps:u32) -> Self {
        let mut init = init();
        let dimensions = dimensions.into();
        init.width(dimensions.x.floor() as i32);
        init.height(dimensions.y.floor() as i32);
        init.title(title.as_ref());
        let (mut handle, thread) = init.build();
        handle.set_target_fps(fps);
        Self { handle,thread }
    }
    /// Creates the window mainloop and closes if the escape or the close button is hit.
    pub fn mainloop(&mut self,main_loop:impl Fn(renderer::Renderer)) {
        loop {
            if self.should_close() { return; }
            main_loop(self.get_renderer());
        }
    }
    /// Creates the window mainloop and only closes if the close button is hit.
    pub fn mainloop_ignore_escape(&mut self,main_loop:impl Fn(renderer::Renderer)) {
        self.handle.set_exit_key(None);
        loop {
            if self.should_close() { return; }
            main_loop(self.get_renderer());
        }
    }
    pub fn get_renderer<'a>(&'a mut self) -> renderer::Renderer {
        let drawer = self.handle.begin_drawing(&self.thread);
        renderer::Renderer::new::<'a>(drawer)
    }
    pub fn should_close(&self) -> bool {
        self.handle.window_should_close()
    }
}
