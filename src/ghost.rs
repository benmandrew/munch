use crate::actor;
use crate::maze;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Personality {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Ghost {
    pub actor: actor::Actor,
    pub personality: Personality,
}

const POSSIBLE_DIRECTIONS: [actor::Direction; 4] = [
    actor::Direction::Left,
    actor::Direction::Right,
    actor::Direction::Up,
    actor::Direction::Down,
];

impl Ghost {
    pub fn new(x: usize, y: usize, personality: Personality) -> Ghost {
        Ghost {
            actor: actor::Actor::new(x, y),
            personality,
        }
    }

    pub fn generate_next_tile(
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
        self.generate_next_tile_with_target(maze, &target);
    }

    fn generate_next_tile_with_target(&mut self, maze: &maze::Maze, target: &(usize, usize)) {
        let ghost_pos = self.actor.get_pos();
        let next_pos_with_dirs = POSSIBLE_DIRECTIONS
            .iter()
            .filter_map(|&dir| {
                if dir == actor::reverse_dir(self.actor.move_direction) {
                    None
                } else {
                    Some((next_pos_from_direction(dir, ghost_pos), dir))
                }
            })
            .collect::<Vec<_>>();
        let mut min_distance_sqr = 0;
        for (next_pos, dir) in next_pos_with_dirs {
            if maze.is_ghost_passable(next_pos.0, next_pos.1) {
                let d = dist_sqr(&next_pos, target);
                if d < min_distance_sqr || min_distance_sqr == 0 {
                    min_distance_sqr = d;
                    self.actor.move_direction = dir;
                }
            }
        }
        if min_distance_sqr == 0 {
            // No valid moves, so reverse direction
            self.actor.move_direction = actor::reverse_dir(self.actor.move_direction);
        }
    }

    pub fn move_along_path(
        &mut self,
        maze: &maze::Maze,
        munch: &actor::Actor,
        blinky_pos: (usize, usize),
        time_delta: f32,
    ) {
        let changed_discrete_position =
            self.actor
                .walk_no_collisions(self.actor.move_direction, maze, time_delta);
        if changed_discrete_position {
            self.generate_next_tile(maze, munch, blinky_pos);
        }
    }
}

fn next_pos_from_direction(dir: actor::Direction, ghost_pos: (usize, usize)) -> (usize, usize) {
    match dir {
        actor::Direction::Up => (ghost_pos.0, ghost_pos.1.wrapping_sub(1)),
        actor::Direction::Down => (ghost_pos.0, ghost_pos.1.wrapping_add(1)),
        actor::Direction::Left => (ghost_pos.0.wrapping_sub(1), ghost_pos.1),
        actor::Direction::Right => (ghost_pos.0.wrapping_add(1), ghost_pos.1),
        actor::Direction::Still => ghost_pos,
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

/// Squared distance between two points
fn dist_sqr(a: &(usize, usize), b: &(usize, usize)) -> u32 {
    let dx = a.0 as isize - b.0 as isize;
    let dy = a.1 as isize - b.1 as isize;
    (dx * dx + dy * dy) as u32
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
mod tests {
    use super::*;

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
