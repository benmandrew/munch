use ggez::graphics::{self, Color, Image};
use ggez::{Context, GameResult};

use ggez::glam;

use crate::maze;

pub struct Window {
    image: Image,
}

impl Window {
    pub fn new(ctx: &mut Context) -> Window {
        let image = match Image::from_path(ctx, "/pacman.bmp") {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to load image: {}", e);
                std::process::exit(1);
            }
        };
        Window { image }
    }

    pub fn draw(&mut self, ctx: &mut Context, maze: &maze::Maze) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        for (i, row) in maze.iter().enumerate() {
            let x = i % maze.width;
            let y = i / maze.width;
            if *row == maze::Tile::Path {
                continue;
            }
            let dest = glam::vec2(x as f32 * 32.0, y as f32 * 32.0);
            canvas.draw(&self.image, graphics::DrawParam::default().dest(dest))
        }
        canvas.finish(ctx)
    }
}
