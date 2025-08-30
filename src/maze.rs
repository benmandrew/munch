use pathfinding::directed::astar;

use crate::actor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Path,
    PlayerImpassable,
    Dot,
}

pub fn player_passable(tile: &Tile) -> bool {
    matches!(tile, Tile::Path | Tile::Dot)
}

pub fn ghost_passable(tile: &Tile) -> bool {
    matches!(tile, Tile::Path | Tile::PlayerImpassable | Tile::Dot)
}

/// Check if two positions are equal considering the maze wrapping
fn pos_mod_eq(a: &(usize, usize), b: &(usize, usize), width: usize, height: usize) -> bool {
    (a.0 % width) == (b.0 % width) && (a.1 % height) == (b.1 % height)
}

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    maze: Vec<Tile>,
}

impl Maze {
    pub fn new(width: usize, height: usize, maze: Vec<Tile>) -> Self {
        Maze {
            width,
            height,
            maze,
        }
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Maze {
            width: 0,
            height: 0,
            maze: Vec::new(),
        }
    }

    pub fn iter(&self) -> MazeIterator<'_> {
        MazeIterator {
            maze: self,
            current: 0,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn is_player_passable(&self, x: usize, y: usize) -> bool {
        player_passable(
            self.maze
                .get(self.index(x % self.width, y % self.height))
                .unwrap(),
        )
    }

    pub fn is_ghost_passable(&self, x: usize, y: usize) -> bool {
        ghost_passable(
            self.maze
                .get(self.index(x % self.width, y % self.height))
                .unwrap(),
        )
    }

    pub fn eat_dots(&mut self, munch: &actor::Actor) -> usize {
        let covering_tiles = munch.get_covering_tiles(0.45);
        let mut eaten = 0;
        for (x, y) in covering_tiles {
            if self.maze.get(self.index(x, y)) == Some(&Tile::Dot) {
                eaten += 1;
                let index = self.index(x, y);
                self.maze[index] = Tile::Path;
            }
        }
        eaten
    }

    fn ghost_successor_tiles(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut successors = Vec::new();
        let (x, y) = *pos;
        let x = x % self.width;
        let y = y % self.height;
        // Check all four directions
        let left = if y > 0 {
            (x, y - 1)
        } else {
            (x, self.height - 1)
        };
        if self.is_ghost_passable(left.0, left.1) {
            successors.push(left);
        }
        if self.is_ghost_passable(x + 1, y) {
            successors.push((x + 1, y));
        }
        if self.is_ghost_passable(x, y + 1) {
            successors.push((x, y + 1));
        }
        let up = if x > 0 {
            (x - 1, y)
        } else {
            (self.width - 1, y)
        };
        if self.is_ghost_passable(up.0, up.1) {
            successors.push(up);
        }
        successors
    }

    fn ghost_successors_with_cost(&self, pos: &(usize, usize)) -> Vec<((usize, usize), u32)> {
        self.ghost_successor_tiles(pos)
            .into_iter()
            .map(|pos| (pos, 1)) // Uniform cost of 1 for each move
            .collect()
    }

    pub fn shortest_path(
        &self,
        start: &(usize, usize),
        goal: &(usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let heuristic = |&(x, y): &(usize, usize)| {
            (((goal.0 % self.width) as isize - x as isize).abs()
                + ((goal.1 % self.height) as isize - y as isize).abs()) as u32
        };
        let result = astar::astar(
            start,
            |pos: &(usize, usize)| self.ghost_successors_with_cost(pos),
            heuristic,
            |pos: &(usize, usize)| pos_mod_eq(pos, goal, self.width, self.height),
        );
        result.map(|(path, _cost)| path)
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut line = String::with_capacity(self.width);
        for y in 0..self.height {
            line.clear();
            for x in 0..self.width {
                let tile = self.maze[self.index(x, y)];
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Path => ' ',
                    Tile::PlayerImpassable => '=',
                    Tile::Dot => '.',
                };
                line.push(c);
            }
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub struct MazeIterator<'a> {
    maze: &'a Maze,
    current: usize,
}

impl<'a> Iterator for MazeIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.maze.maze.len() {
            let tile = &self.maze.maze[self.current];
            self.current += 1;
            Some(tile)
        } else {
            None
        }
    }
}

#[cfg(test)]
impl Maze {
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.maze[y * self.width + x])
        } else {
            None
        }
    }
}

#[cfg(test)]
use crate::config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_to_string() {
        let maze_str = "
#####
#   #
#=#=#
#...#
#####
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        let maze_display = maze.to_string();
        pretty_assertions::assert_eq!(maze_display.trim(), maze_str.trim());
    }
}
