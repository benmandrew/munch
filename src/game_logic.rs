use ggez::input::keyboard::KeyCode;
use ggez::GameResult;

use crate::{actor, ghost, maze};

pub struct GameLogic {
    pub maze: maze::Maze,
    pub munch: actor::Actor,
    pub ghosts: Vec<ghost::Ghost>,
    move_direction: actor::Direction,
    pub score: u32,
}

/// Represents either a maze or a path to a file containing a maze.
pub enum MazeOrPath {
    Maze(maze::Maze),
    Path(String),
}

const DEFAULT_MAZE_PATH: &str = "resources/maze.txt";

impl GameLogic {
    /// Creates a new GameLogic instance.
    /// If no maze or path is provided, the default maze will be loaded.
    pub fn new(maze_or_path: Option<MazeOrPath>) -> GameLogic {
        let maze = match maze_or_path {
            Some(MazeOrPath::Maze(m)) => m,
            Some(MazeOrPath::Path(path)) => get_maze_from_file(&path),
            None => get_maze_from_file(DEFAULT_MAZE_PATH),
        };
        let munch = actor::Actor::new(10, 16);
        let ghosts: Vec<ghost::Ghost> = vec![];
        GameLogic {
            maze,
            munch,
            ghosts,
            move_direction: actor::Direction::Still,
            score: 0,
        }
    }

    pub fn handle_movement(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => self.move_direction = actor::Direction::Up,
            KeyCode::Down => self.move_direction = actor::Direction::Down,
            KeyCode::Left => self.move_direction = actor::Direction::Left,
            KeyCode::Right => self.move_direction = actor::Direction::Right,
            _ => {}
        }
    }

    /// If there is a collision between the munch and a ghost,
    /// return the index of the ghost. A collision is considered to have
    /// occurred if the positions of the munch and the ghost are the same.
    fn munch_ghost_collision(&self) -> Option<usize> {
        for (i, ghost) in self.ghosts.iter().enumerate() {
            if self.munch.get_pos() == ghost.actor.get_pos() {
                return Some(i);
            }
        }
        None
    }

    pub fn update(&mut self, time_delta: f32) -> GameResult {
        self.munch.walk(self.move_direction, &self.maze, time_delta);
        let dots_eaten = self.maze.eat_dots(&self.munch);
        self.score += dots_eaten as u32 * 10;
        for ghost in &mut self.ghosts {
            ghost.move_along_path(&self.maze, &(self.munch.get_pos()), time_delta);
        }
        Ok(())
    }
}

fn get_maze_from_file(path: &str) -> maze::Maze {
    match maze::Maze::from_file(path) {
        Ok(maze) => maze,
        Err(e) => {
            eprintln!("Error loading maze: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
fn init_empty() -> GameLogic {
    GameLogic::new(Some(MazeOrPath::Maze(maze::Maze::empty())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_munch_ghost_collisions() {
        let mut game = init_empty();
        game.munch.set_pos(5, 5);
        game.ghosts.push(ghost::Ghost::new(5, 5));
        assert_eq!(game.munch_ghost_collision(), Some(0));
    }
}
