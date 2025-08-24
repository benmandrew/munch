use sdl2::event;
use sdl2::render;
use sdl2::video;
use sdl2::EventPump;

pub struct Window {
    pub canvas: render::Canvas<video::Window>,
    event_pump: EventPump,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Munch", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Window { canvas, event_pump }
    }

    pub fn event_poll_iter(&mut self) -> Vec<event::Event> {
        self.event_pump.poll_iter().collect()
    }
}

pub struct TextureStore<'a> {
    texture_creator: render::TextureCreator<video::WindowContext>,
    textures: std::collections::HashMap<String, render::Texture<'a>>,
}

impl<'a> TextureStore<'a> {
    pub fn new(texture_creator: render::TextureCreator<video::WindowContext>) -> Self {
        TextureStore {
            texture_creator,
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_bmp(&'a mut self, name: &String, path: &std::path::Path) {
        let surface = sdl2::surface::Surface::load_bmp(path).unwrap();
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        self.textures.insert(name.clone(), texture);
    }
}
