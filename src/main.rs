extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time;

const FRAME_TIME: time::Duration = time::Duration::from_nanos(1_000_000_000u64 / 60);

mod window;

pub fn main() {
    let mut win = window::Window::new(800, 600);
    win.canvas.set_draw_color(Color::RGB(0, 255, 255));
    win.canvas.clear();
    win.canvas.present();
    let mut i = 0;
    'running: loop {
        let start = time::Instant::now();
        i = (i + 1) % 255;
        win.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        win.canvas.clear();
        for event in win.event_poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        win.canvas.present();
        if start.elapsed() < FRAME_TIME {
            ::std::thread::sleep(FRAME_TIME - start.elapsed());
        }
    }
}
