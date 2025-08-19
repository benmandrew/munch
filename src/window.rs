pub struct Window {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
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

    pub fn event_poll_iter(&mut self) -> Vec<sdl2::event::Event> {
        self.event_pump.poll_iter().collect()
    }
}
