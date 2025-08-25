use ggez::graphics::{self, Canvas, Color, Image};
use ggez::{Context, GameResult};

use ggez::glam;

use crate::maze;
use crate::munch;

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

    fn draw_maze(&self, canvas: &mut Canvas, maze: &maze::Maze) -> (f32, f32) {
        let phys_maze_width = maze.width as f32 * SCALE;
        let phys_maze_height = maze.height as f32 * SCALE;
        let start_x = (self.width - phys_maze_width) / 2.0;
        let start_y = (self.height - phys_maze_height) / 2.0;
        for (i, row) in maze.iter().enumerate() {
            let x = i % maze.width;
            let y = i / maze.width;
            let colour = match row {
                maze::Tile::Wall => Color::BLUE,
                maze::Tile::PlayerImpassable => Color::WHITE,
                maze::Tile::Path => continue,
            };
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
                    .color(colour),
            );
        }
        (start_x, start_y)
    }

    fn draw_munch(&self, canvas: &mut Canvas, munch: &munch::Munch, start_x: f32, start_y: f32) {
        let (munch_x, munch_y) = munch.get_draw_pos();
        let pos = glam::Vec2::new(munch_x * SCALE + start_x, munch_y * SCALE + start_y);
        canvas.draw(&self.image, graphics::DrawParam::new().dest(pos));
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        maze: &maze::Maze,
        munch: &munch::Munch,
    ) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let (start_x, start_y) = self.draw_maze(&mut canvas, maze);
        self.draw_munch(&mut canvas, munch, start_x, start_y);
        canvas.finish(ctx)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
