use crate::window;

pub struct Game<'a> {
    pub win: window::Window,
    pub texture_store: window::TextureStore<'a>,
}

impl<'a> Game<'a> {
    pub fn new(width: usize, height: usize) -> Self {
        let win = window::Window::new(width, height);
        let texture_creator = win.canvas.texture_creator();
        let texture_store = window::TextureStore::new(texture_creator);
        Game { win, texture_store }
    }

    pub fn load_texture(&'a mut self, name: &String, path: &std::path::Path) {
        self.texture_store.load_bmp(name, path);
    }
}
