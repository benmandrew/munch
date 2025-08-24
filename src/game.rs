use ggez::event::EventHandler;
use ggez::{Context, GameResult};

use crate::maze;
use crate::window;

pub struct Game {
    window: window::Window,
    maze: maze::Maze,
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
        Game { window, maze }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.window.draw(ctx, &self.maze)
    }
}
