use log::warn;

use crate::actor;
use crate::maze;

fn pair_to_direction(from: &(usize, usize), to: &(usize, usize)) -> actor::Direction {
    let dx = to.0 as isize - from.0 as isize;
    let dy = to.1 as isize - from.1 as isize;
    if dx == 0 && dy == 0 {
        warn!(
            "pair_to_direction called with identical positions: {:?}",
            from
        );
        return actor::Direction::Still;
    }
    if dx.abs() > dy.abs() {
        if dx > 0 {
            actor::Direction::Right
        } else {
            actor::Direction::Left
        }
    } else if dy > 0 {
        actor::Direction::Down
    } else {
        actor::Direction::Up
    }
}

pub struct Ghost {
    pub actor: actor::Actor,
    path: Vec<(usize, usize)>,
    path_index: usize,
}

impl Ghost {
    pub fn new(x: usize, y: usize) -> Ghost {
        Ghost {
            actor: actor::Actor::new(x, y),
            path: Vec::new(),
            path_index: 0,
        }
    }

    pub fn generate_path(&mut self, maze: &maze::Maze, target: &(usize, usize)) {
        let ghost_pos = self.actor.get_pos();
        match maze.shortest_path(&ghost_pos, target) {
            Some(path) => {
                self.path = path;
                self.path_index = 0;
            }
            None => warn!(
                "No path found for ghost from {:?} to {:?}",
                ghost_pos, target
            ),
        }
    }

    pub fn move_along_path(&mut self, maze: &maze::Maze, target: &(usize, usize), time_delta: f32) {
        if self.path_index >= self.path.len() - 1 || self.path.is_empty() {
            self.generate_path(maze, target);
            if self.path.is_empty() {
                return;
            }
        }
        let direction =
            pair_to_direction(&self.path[self.path_index], &self.path[self.path_index + 1]);
        let changed_discrete_position = self.actor.walk_no_collisions(direction, maze, time_delta);
        if changed_discrete_position {
            self.path_index += 1;
            warn!("Last direction was {:?}", direction);
        }
    }
}
