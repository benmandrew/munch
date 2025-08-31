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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Personality {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Ghost {
    pub actor: actor::Actor,
    path: Vec<(usize, usize)>,
    path_index: usize,
    pub personality: Personality,
}

impl Ghost {
    pub fn new(x: usize, y: usize, personality: Personality) -> Ghost {
        Ghost {
            actor: actor::Actor::new(x, y),
            path: Vec::new(),
            path_index: 0,
            personality,
        }
    }

    pub fn generate_path(
        &mut self,
        maze: &maze::Maze,
        munch: &actor::Actor,
        blinky_pos: (usize, usize),
    ) {
        let target = match self.personality {
            Personality::Blinky => get_blinky_target(munch),
            Personality::Pinky => get_pinky_target(munch, maze),
            Personality::Inky => get_inky_target(munch, maze, blinky_pos),
            Personality::Clyde => get_clyde_target(munch),
        };
        self.generate_path_to_target(maze, &target);
    }

    fn generate_path_to_target(&mut self, maze: &maze::Maze, target: &(usize, usize)) {
        let ghost_pos = self.actor.get_pos();
        match maze.shortest_path(&ghost_pos, target) {
            Some(path) => {
                self.path = path;
                self.path_index = 0;
            }
            None => warn!(
                "No path found for {:?} from {:?} to {:?}",
                self.personality, ghost_pos, target
            ),
        }
    }

    pub fn move_along_path(
        &mut self,
        maze: &maze::Maze,
        munch: &actor::Actor,
        blinky_pos: (usize, usize),
        time_delta: f32,
    ) {
        if self.path_index + 1 >= self.path.len() || self.path.len() <= 1 {
            self.generate_path(maze, munch, blinky_pos);
            if self.path.len() <= 1 {
                return;
            }
        }
        let direction =
            pair_to_direction(&self.path[self.path_index], &self.path[self.path_index + 1]);
        let changed_discrete_position = self.actor.walk_no_collisions(direction, maze, time_delta);
        if changed_discrete_position {
            // self.path_index += 1;
            self.generate_path(maze, munch, blinky_pos);
        }
    }
}

/// Blinky directly targets the player's current position.
fn get_blinky_target(munch: &actor::Actor) -> (usize, usize) {
    munch.get_pos()
}

/// Pinky tries to move towards the tile four spaces ahead of the player.
fn get_lookahead_target(
    munch: &actor::Actor,
    maze: &maze::Maze,
    lookahead: usize,
) -> (usize, usize) {
    let (mut x, mut y) = munch.get_pos();
    for i in (1..lookahead + 1).rev() {
        match munch.move_direction {
            actor::Direction::Up => {
                if y < i {
                    y += maze.height;
                }
                if maze.is_ghost_passable(x, y - i) {
                    return (x, y - i);
                }
            }
            actor::Direction::Down => {
                if maze.is_ghost_passable(x, y + i) {
                    return (x, y + i);
                }
            }
            actor::Direction::Left => {
                if x < i {
                    x += maze.width;
                }
                if maze.is_ghost_passable(x - i, y) {
                    return (x - i, y);
                }
            }
            actor::Direction::Right => {
                if maze.is_ghost_passable(x + i, y) {
                    return (x + i, y);
                }
            }
            _ => {}
        }
    }
    (x % maze.width, y % maze.height)
}

const PINKY_LOOKAHEAD: usize = 4;

fn get_pinky_target(munch: &actor::Actor, maze: &maze::Maze) -> (usize, usize) {
    get_lookahead_target(munch, maze, PINKY_LOOKAHEAD)
}

const INKY_LOOKAHEAD: usize = 2;

/// Inky targets a position based on the player's position and Blinky's position.
fn get_inky_target(
    munch: &actor::Actor,
    maze: &maze::Maze,
    blinky_pos: (usize, usize),
) -> (usize, usize) {
    let (mut x, mut y) = get_lookahead_target(munch, maze, INKY_LOOKAHEAD);
    if x < blinky_pos.0 {
        x += maze.width + x - blinky_pos.0;
    } else {
        x += x - blinky_pos.0;
    }
    if y < blinky_pos.1 {
        y += maze.height + y - blinky_pos.1;
    } else {
        y += y - blinky_pos.1;
    }
    (x % maze.width, y % maze.height)
}

fn get_clyde_target(munch: &actor::Actor) -> (usize, usize) {
    munch.get_pos()
}

#[cfg(test)]
use crate::config;

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
        let mut ghost = Ghost::new(1, 1, Personality::Blinky);
        let maze_str = "
#####
#   #
##=##
#...#
#####
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        ghost.generate_path_to_target(&maze, &(3, 3));
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
        let mut ghost = Ghost::new(0, 1, Personality::Inky);
        let maze_str = "
####
 #  
####
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        ghost.generate_path_to_target(&maze, &(2, 1));
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

    #[test]
    fn test_pinky_target() {
        let maze_str = "
###########
#         #
#         #
#         #
###########
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        let mut munch = actor::Actor::new(4, 1);
        munch.move_direction = actor::Direction::Right;
        let target = get_pinky_target(&munch, &maze);
        pretty_assertions::assert_eq!(target, (4 + PINKY_LOOKAHEAD, 1));
        munch.move_direction = actor::Direction::Down;
        let target = get_pinky_target(&munch, &maze);
        pretty_assertions::assert_eq!(target, (4, 3));
    }

    #[test]
    fn test_inky_target() {
        let maze_str = "
###########
#         #
#         #
#         #
#         #
#         #
###########
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        let mut munch = actor::Actor::new(4, 3);
        munch.move_direction = actor::Direction::Right;
        let blinky_pos = (2, 2);
        let target = get_inky_target(&munch, &maze, blinky_pos);
        pretty_assertions::assert_eq!(target, (10, 4));
        let mut munch = actor::Actor::new(4, 1);
        munch.move_direction = actor::Direction::Up;
        let blinky_pos = (1, 4);
        let target = get_inky_target(&munch, &maze, blinky_pos);
        pretty_assertions::assert_eq!(target, (7, 5));
    }
}
