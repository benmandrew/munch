use ggez::graphics::{self, Color, Image};
use ggez::{Context, GameResult};

use ggez::glam;

use crate::maze;

const SCALE: f32 = 32.0;

pub struct Window {
    image: Image,
    width: f32,
    height: f32,
}

impl Window {
    pub fn new(ctx: &mut Context) -> Window {
        let size = ctx.gfx.window().inner_size();
        let image = match Image::from_path(ctx, "/pacman.bmp") {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to load image: {}", e);
                std::process::exit(1);
            }
        };
        Window {
            image,
            width: size.width as f32,
            height: size.height as f32,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, maze: &maze::Maze) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let phys_maze_width = maze.width as f32 * SCALE;
        let phys_maze_height = maze.height as f32 * SCALE;
        let start_x = (self.width - phys_maze_width) / 2.0;
        let start_y = (self.height - phys_maze_height) / 2.0;
        for (i, row) in maze.iter().enumerate() {
            let x = i % maze.width;
            let y = i / maze.width;
            if *row == maze::Tile::Path {
                continue;
            }
            let rect = graphics::Rect::new(
                start_x + x as f32 * SCALE,
                start_y + y as f32 * SCALE,
                SCALE,
                SCALE,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(Color::BLUE),
            );
        }
        canvas.finish(ctx)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
