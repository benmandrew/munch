use ggez::error::GameError;
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

use crate::maze;
use crate::munch;
use crate::window;

const FRAME_TIME: f32 = 1000.0 / 120.0;

pub struct Game {
    window: window::Window,
    maze: maze::Maze,
    munch: munch::Munch,
    spin_sleep: spin_sleep::SpinSleeper,
    last_game_update: std::time::Instant,
    move_direction: munch::Direction,
    score: u32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let window = window::Window::new(ctx);
        let maze = match maze::Maze::from_file("resources/maze.txt") {
            Ok(maze) => maze,
            Err(e) => {
                eprintln!("Error loading maze: {}", e);
                std::process::exit(1);
            }
        };
        let munch = munch::Munch::new(1, 1);
        let spin_sleep = spin_sleep::SpinSleeper::new(100_000)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);
        Game {
            window,
            maze,
            munch,
            spin_sleep,
            last_game_update: std::time::Instant::now(),
            move_direction: munch::Direction::Still,
            score: 0,
        }
    }
}

impl Game {
    fn handle_movement(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => self.move_direction = munch::Direction::Up,
            KeyCode::Down => self.move_direction = munch::Direction::Down,
            KeyCode::Left => self.move_direction = munch::Direction::Left,
            KeyCode::Right => self.move_direction = munch::Direction::Right,
            _ => {}
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.spin_sleep.sleep_until(
            self.last_game_update + std::time::Duration::from_millis(FRAME_TIME as u64),
        );

        let time_delta = self.last_game_update.elapsed().as_millis() as f32 / 1000.0;

        self.munch.walk(self.move_direction, &self.maze, time_delta);

        self.last_game_update = std::time::Instant::now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        let keycode = match input.keycode {
            Some(key) => key,
            None => return Ok(()),
        };
        self.handle_movement(keycode);
        match keycode {
            KeyCode::Escape | KeyCode::Q => {
                ctx.request_quit();
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.window.draw(ctx, &self.maze, &self.munch, self.score)
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        width: f32,
        height: f32,
    ) -> Result<(), GameError> {
        self.window.resize(width, height);
        Ok(())
    }
}
