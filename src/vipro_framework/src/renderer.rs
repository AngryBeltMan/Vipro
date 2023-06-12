use raylib::prelude::*;
pub struct Renderer<'a> {
    drawer: RaylibDrawHandle<'a>,
}
impl<'a> Renderer<'a> {
    pub fn new<'b>(drawer: RaylibDrawHandle<'a>) -> Self
        where 'b:'a
    {
        Self { drawer }
    }
    /// Unsafe because only the library should be calling this function
    pub unsafe fn get_drawer(&self) -> &RaylibDrawHandle { &self.drawer }

    // LifeTimes: the returned reference is valid as long as the Drawhandle is valid
    // 'a is just a anonymous lifetime
    /// Unsafe because only the library should be calling this function
    pub unsafe fn get_drawer_mut<'b>(&'b mut self) -> &'b mut RaylibDrawHandle<'a> { &mut self.drawer }
    /// Returns true if the window should close
    /// ie: The Escape key is pressed or the close button is hit
    pub fn should_close(&self) -> bool {
        self.drawer.window_should_close()
    }
    pub fn set_background(&mut self,r:u8,g:u8,b:u8) {
        self.drawer.clear_background(Color::new(r, g, b, 255));
    }
}
