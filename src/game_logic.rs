use core::panic;

use ggez::input::keyboard::KeyCode;
use ggez::GameResult;

use crate::{actor, config, ghost, maze};

const ENERGISED_TIME: f32 = 10.0;

/// Has Munch eaten a power pellet recently?
/// If so, the ghosts can be eaten.
struct Energised {
    is_energised: bool,
    timer: f32,
}

impl Energised {
    fn new() -> Self {
        Energised {
            is_energised: false,
            timer: 0.0,
        }
    }

    fn update(
        &mut self,
        ghosts: &mut Vec<ghost::Ghost>,
        power_pellets_eaten: i32,
        time_delta: f32,
    ) {
        if power_pellets_eaten > 0 {
            log::info!("Munch is energised");
            self.timer = ENERGISED_TIME;
            self.is_energised = true;
            for ghost in ghosts {
                ghost.set_mode_scatter();
            }
        } else if self.timer > 0.0 {
            self.timer -= time_delta;
        } else if self.is_energised {
            log::info!("Munch is no longer energised");
            self.is_energised = false;
            for ghost in ghosts {
                ghost.set_mode_chase();
            }
        }
    }
}

pub struct GameLogic {
    pub maze: maze::Maze,
    pub munch: actor::Actor,
    pub ghosts: Vec<ghost::Ghost>,
    move_direction: actor::Direction,
    energised: Energised,
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
                .map(|&(x, y, personality)| ghost::Ghost::new(x, y, personality))
                .collect(),
            move_direction: actor::Direction::Still,
            energised: Energised::new(),
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

    fn get_blinky_pos(&self) -> (i32, i32) {
        self.ghosts
            .iter()
            .find(|g| matches!(g.personality, ghost::Personality::Blinky))
            .map(|g| g.actor.get_pos())
            .unwrap_or((0, 0))
    }

    fn add_score(&mut self, dots_eaten: i32, power_pellets_eaten: i32) {
        self.score += dots_eaten as u32 * 10 + power_pellets_eaten as u32 * 50;
    }

    fn handle_eating(&mut self, time_delta: f32) {
        let dots_eaten = self.maze.eat_dots(&self.munch);
        let power_pellets_eaten = self.maze.eat_power_pellets(&self.munch);
        self.energised
            .update(&mut self.ghosts, power_pellets_eaten, time_delta);
        self.add_score(dots_eaten, power_pellets_eaten);
    }

    fn handle_ghost_movement(&mut self, time_delta: f32) {
        let blinky_pos = self.get_blinky_pos();
        for ghost in &mut self.ghosts {
            ghost.move_along_path(&self.maze, &self.munch, blinky_pos, time_delta);
            if ghost.actor.get_pos() == (self.maze.width / 2, self.maze.height / 2 - 1)
                && ghost.mode == ghost::Mode::Eaten
            {
                log::info!("{:?} has respawned", ghost.personality);
                ghost.mode = ghost::Mode::Chase;
            }
        }
    }

    fn handle_ghost_collision(&mut self, ghost_index: usize) {
        if self.energised.is_energised {
            if self.ghosts[ghost_index].eat_ghost() {
                log::info!("Munch has eaten {:?}", self.ghosts[ghost_index].personality);
                self.score += 200;
            }
        } else {
            log::info!(
                "Munch collided with a {:?}.",
                self.ghosts[ghost_index].personality
            );
            panic!("Munch collided with a ghost!");
        }
    }

    pub fn update(&mut self, time_delta: f32) -> GameResult {
        self.munch.walk(self.move_direction, &self.maze, time_delta);
        self.handle_eating(time_delta);
        self.handle_ghost_movement(time_delta);
        if let Some(index) = self.munch_ghost_collision() {
            self.handle_ghost_collision(index);
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
        game.ghosts
            .push(ghost::Ghost::new(5, 5, ghost::Personality::Blinky));
        assert_eq!(game.munch_ghost_collision(), Some(0));
    }
}
