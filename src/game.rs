use ggez::error::GameError;
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

use crate::maze;
use crate::window;

struct Munch {
    x: usize,
    y: usize,
}

pub struct Game {
    window: window::Window,
    maze: maze::Maze,
    munch: Munch,
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
        let munch = Munch { x: 1, y: 1 };
        Game {
            window,
            maze,
            munch,
        }
    }
}

impl Game {
    fn handle_movement(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                if !self
                    .maze
                    .is_wall(self.munch.x, self.munch.y + self.maze.height - 1)
                {
                    if self.munch.y == 0 {
                        self.munch.y = self.maze.height - 1;
                    } else {
                        self.munch.y -= 1;
                    }
                }
            }
            KeyCode::Down => {
                if !self.maze.is_wall(self.munch.x, self.munch.y + 1) {
                    if self.munch.y == self.maze.height - 1 {
                        self.munch.y = 0;
                    } else {
                        self.munch.y += 1;
                    }
                }
            }
            KeyCode::Left => {
                if !self
                    .maze
                    .is_wall(self.munch.x + self.maze.width - 1, self.munch.y)
                {
                    if self.munch.x == 0 {
                        self.munch.x = self.maze.width - 1;
                    } else {
                        self.munch.x -= 1;
                    }
                }
            }
            KeyCode::Right => {
                if !self.maze.is_wall(self.munch.x + 1, self.munch.y) {
                    if self.munch.x == self.maze.width - 1 {
                        self.munch.x = 0;
                    } else {
                        self.munch.x += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
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
        self.window
            .draw(ctx, &self.maze, self.munch.x, self.munch.y)
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
