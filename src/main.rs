extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time;

const FRAME_TIME: time::Duration = time::Duration::from_nanos(1_000_000_000u64 / 60);

mod game;
mod window;

pub fn main() {
    let mut g = game::Game::new(800, 600);

    g.win.canvas.set_draw_color(Color::RGB(0, 255, 255));
    g.win.canvas.clear();
    g.win.canvas.present();
    g.load_texture(
        &String::from("pacman"),
        &std::path::Path::new("resources/pacman.bmp"),
    );
    // let mut i = 0;
    // 'running: loop {
    //     let start = time::Instant::now();
    //     i = (i + 1) % 255;
    //     g.win.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     g.win.canvas.clear();
    //     for event in g.win.event_poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...
    //     // win.canvas.copy(&texture, None, None);
    //     g.win.canvas.present();
    //     if start.elapsed() < FRAME_TIME {
    //         ::std::thread::sleep(FRAME_TIME - start.elapsed());
    //     }
    // }
}
