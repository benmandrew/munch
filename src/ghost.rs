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
        if self.path_index >= self.path.len() - 1 || self.path.len() <= 1 {
            self.generate_path(maze, target);
            if self.path.len() <= 1 {
                return;
            }
        }
        let direction =
            pair_to_direction(&self.path[self.path_index], &self.path[self.path_index + 1]);
        let changed_discrete_position = self.actor.walk_no_collisions(direction, maze, time_delta);
        if changed_discrete_position {
            self.path_index += 1;
        }
    }
}

#[cfg(test)]
fn mark_maze(maze: &maze::Maze, path: Vec<(usize, usize)>) -> String {
    let maze_str = maze.to_string();
    let mut result = String::with_capacity(maze_str.len());
    let mut row_idx = 0;
    for (i, c) in maze_str.chars().enumerate() {
        if c == '\n' {
            row_idx += 1;
        }
        if path.contains(&(i % (maze.width + 1), row_idx)) {
            result.push('!');
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_ghost_path_through_player_impassable_tile() {
        let mut ghost = Ghost::new(1, 1);
        let maze_str = "
#####
#   #
##=##
#...#
#####
";
        let maze = maze::Maze::from_string(maze_str).unwrap();
        ghost.generate_path(&maze, &(3, 3));
        pretty_assertions::assert_eq!(ghost.path.len(), 5);
        pretty_assertions::assert_eq!(ghost.path, vec![(1, 1), (2, 1), (2, 2), (2, 3), (3, 3)]);
        let marked = mark_maze(&maze, ghost.path.clone());
        let expected = "
#####
#!! #
##!##
#.!!#
#####
";
        pretty_assertions::assert_eq!(marked.trim(), expected.trim());
    }

    #[test]
    fn test_ghost_path_wraparound() {
        let mut ghost = Ghost::new(0, 1);
        let maze_str = "
####
 #  
####
";
        let maze = maze::Maze::from_string(maze_str).unwrap();
        ghost.generate_path(&maze, &(2, 1));
        pretty_assertions::assert_eq!(ghost.path.len(), 3);
        pretty_assertions::assert_eq!(ghost.path, vec![(0, 1), (3, 1), (2, 1)]);
        let marked = mark_maze(&maze, ghost.path.clone());
        let expected = "
####
!#!!
####
";
        pretty_assertions::assert_eq!(marked.trim(), expected.trim());
    }
}
