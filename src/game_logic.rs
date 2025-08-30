use ggez::input::keyboard::KeyCode;
use ggez::GameResult;

use crate::{actor, config, ghost, maze};

pub struct GameLogic {
    pub maze: maze::Maze,
    pub munch: actor::Actor,
    pub ghosts: Vec<ghost::Ghost>,
    move_direction: actor::Direction,
    pub score: u32,
}

impl GameLogic {
    pub fn new(config: config::Config) -> GameLogic {
        let munch = match config.player_pos {
            Some((x, y)) => actor::Actor::new(x, y),
            None => actor::Actor::new(0, 0),
        };
        GameLogic {
            maze: config.maze,
            munch,
            ghosts: config
                .ghosts_pos
                .iter()
                .map(|&(x, y)| ghost::Ghost::new(x, y))
                .collect(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_munch_ghost_collisions() {
        let mut game = GameLogic::new(config::Config::empty());
        game.munch.set_pos(5, 5);
        game.ghosts.push(ghost::Ghost::new(5, 5));
        assert_eq!(game.munch_ghost_collision(), Some(0));
    }
}
