use ggez::graphics::{self, Canvas, Color, Image, Text};
use ggez::{Context, GameResult};

use ggez::glam;

use crate::maze;
use crate::munch;

const SCALE: f32 = 32.0;
const PELLET_SCALE: f32 = 0.2;

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

    fn draw_wall(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let rect = graphics::Rect::new(x, y, SCALE, SCALE);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::BLUE),
        );
    }

    fn draw_player_impassable(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let rect = graphics::Rect::new(x, y, SCALE, SCALE);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::WHITE),
        );
    }

    fn draw_dot(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let dot_size = SCALE * PELLET_SCALE;
        let offset = (SCALE - dot_size) / 2.0;
        let rect = graphics::Rect::new(x + offset, y + offset, dot_size, dot_size);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::MAGENTA),
        );
    }

    fn draw_maze(&self, canvas: &mut Canvas, maze: &maze::Maze) -> (f32, f32) {
        let phys_maze_width = maze.width as f32 * SCALE;
        let phys_maze_height = maze.height as f32 * SCALE;
        let start_x = (self.width - phys_maze_width) / 2.0;
        let start_y = (self.height - phys_maze_height) / 2.0;
        for (i, tile) in maze.iter().enumerate() {
            let x = (i % maze.width) as f32 * SCALE + start_x;
            let y = (i / maze.width) as f32 * SCALE + start_y;
            match tile {
                maze::Tile::Wall => self.draw_wall(canvas, x, y),
                maze::Tile::PlayerImpassable => self.draw_player_impassable(canvas, x, y),
                maze::Tile::Dot => self.draw_dot(canvas, x, y),
                maze::Tile::Path => continue,
            };
        }
        (start_x, start_y)
    }

    fn draw_munch(&self, canvas: &mut Canvas, munch: &munch::Munch, start_x: f32, start_y: f32) {
        let (munch_x, munch_y) = munch.get_draw_pos();
        let pos = glam::Vec2::new(munch_x * SCALE + start_x, munch_y * SCALE + start_y);
        canvas.draw(&self.image, graphics::DrawParam::new().dest(pos));
    }

    fn draw_fps(&self, ctx: &Context, canvas: &mut Canvas) {
        let fps = ctx.time.fps().round();
        let fps_display = Text::new(format!("FPS: {fps}"));
        canvas.draw(
            &fps_display,
            graphics::DrawParam::from([200.0, 0.0]).color(Color::WHITE),
        );
    }

    fn draw_score(&self, canvas: &mut Canvas, score: u32) {
        let score_display = Text::new(format!("Score: {}", score));
        canvas.draw(
            &score_display,
            graphics::DrawParam::from([200.0, 20.0]).color(Color::WHITE),
        );
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        maze: &maze::Maze,
        munch: &munch::Munch,
        score: u32,
    ) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let (start_x, start_y) = self.draw_maze(&mut canvas, maze);
        self.draw_munch(&mut canvas, munch, start_x, start_y);
        self.draw_fps(ctx, &mut canvas);
        self.draw_score(&mut canvas, score);
        canvas.finish(ctx)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
