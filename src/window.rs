use ggez::graphics::{self, Canvas, Color, Text};
use ggez::{Context, GameResult};

use ggez::glam;

use crate::{actor, config, ghost, maze, spritesheet};

const DOT_SCALE: f32 = 0.2;
const POWER_PELLET_SCALE: f32 = 0.4;

pub struct Window {
    spritesheet: spritesheet::SpriteSheet,
    width: f32,
    height: f32,
    frame: usize,
}

impl Window {
    pub fn new(ctx: &mut Context) -> Window {
        let size = ctx.gfx.window().inner_size();
        Window {
            spritesheet: spritesheet::SpriteSheet::new(ctx),
            width: size.width as f32,
            height: size.height as f32,
            frame: 0,
        }
    }

    fn draw_wall(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let rect = graphics::Rect::new(x, y, config::TILE_SIZE, config::TILE_SIZE);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::BLUE),
        );
    }

    fn draw_player_impassable(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let rect = graphics::Rect::new(x, y, config::TILE_SIZE, config::TILE_SIZE);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::CYAN),
        );
    }

    fn draw_dot(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let dot_size = config::TILE_SIZE * DOT_SCALE;
        let offset = (config::TILE_SIZE - dot_size) / 2.0;
        let rect = graphics::Rect::new(x + offset, y + offset, dot_size, dot_size);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::MAGENTA),
        );
    }

    fn draw_power_pellet(&self, canvas: &mut Canvas, x: f32, y: f32) {
        let dot_size = config::TILE_SIZE * POWER_PELLET_SCALE;
        let offset = (config::TILE_SIZE - dot_size) / 2.0;
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
        let phys_maze_width = maze.width as f32 * config::TILE_SIZE;
        let phys_maze_height = maze.height as f32 * config::TILE_SIZE;
        let start_x = (self.width - phys_maze_width) / 2.0;
        let start_y = (self.height - phys_maze_height) / 2.0;
        for (i, tile) in maze.iter().enumerate() {
            let x = (i % maze.width as usize) as f32 * config::TILE_SIZE + start_x;
            let y = (i / maze.width as usize) as f32 * config::TILE_SIZE + start_y;
            match tile {
                maze::Tile::Wall => self.draw_wall(canvas, x, y),
                maze::Tile::PlayerImpassable => self.draw_player_impassable(canvas, x, y),
                maze::Tile::Dot => self.draw_dot(canvas, x, y),
                maze::Tile::PowerPellet => self.draw_power_pellet(canvas, x, y),
                maze::Tile::Path => continue,
            };
        }
        (start_x, start_y)
    }

    fn draw_munch(&self, canvas: &mut Canvas, munch: &actor::Actor, start_x: f32, start_y: f32) {
        let (munch_x, munch_y) = munch.get_draw_pos();
        let pos = glam::Vec2::new(
            munch_x * config::TILE_SIZE + start_x,
            munch_y * config::TILE_SIZE + start_y,
        );
        self.spritesheet
            .draw_munch(canvas, munch.move_direction, pos, self.frame);
    }

    fn draw_ghost(&self, canvas: &mut Canvas, ghost: &ghost::Ghost, start_x: f32, start_y: f32) {
        let (ghost_x, ghost_y) = ghost.actor.get_draw_pos();
        let pos = glam::Vec2::new(
            ghost_x * config::TILE_SIZE + start_x,
            ghost_y * config::TILE_SIZE + start_y,
        );
        self.spritesheet.draw_ghost(canvas, ghost, pos, self.frame);
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
        munch: &actor::Actor,
        ghosts: &Vec<ghost::Ghost>,
        score: u32,
    ) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let (start_x, start_y) = self.draw_maze(&mut canvas, maze);
        self.draw_munch(&mut canvas, munch, start_x, start_y);
        for ghost in ghosts {
            self.draw_ghost(&mut canvas, ghost, start_x, start_y);
        }
        self.draw_fps(ctx, &mut canvas);
        self.draw_score(&mut canvas, score);
        (self.frame, _) = usize::overflowing_add(self.frame, 1);
        canvas.finish(ctx)
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
