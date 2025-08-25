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

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    maze: Vec<Tile>,
}

impl Maze {
    pub fn from_string(s: &str) -> Result<Self, String> {
        let lines: Vec<&str> = s.trim().split('\n').collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut maze = Vec::with_capacity(width * height);
        for (y, line) in lines.iter().enumerate() {
            if width != line.len() {
                return Err(format!(
                    "Inconsistent line length: line 1 has length {}, line {} has length {}",
                    width,
                    y + 1,
                    line.len()
                ));
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => maze.push(Tile::Wall),
                    ' ' => maze.push(Tile::Path),
                    '=' => maze.push(Tile::PlayerImpassable),
                    '.' => maze.push(Tile::Dot),
                    _ => {
                        return Err(format!("Unknown tile character '{}' at ({}, {})", c, x, y));
                    }
                }
            }
        }
        Ok(Maze {
            width,
            height,
            maze,
        })
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Self::from_string(&contents),
            Err(e) => Err(e.to_string()),
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

    fn is_ghost_passable(&self, x: usize, y: usize) -> bool {
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
            ((goal.0 as isize - x as isize).abs() + (goal.1 as isize - y as isize).abs()) as u32
        };
        let result = astar::astar(
            start,
            |pos: &(usize, usize)| self.ghost_successors_with_cost(pos),
            heuristic,
            |pos: &(usize, usize)| pos == goal,
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
    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.maze[y * self.width + x])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_from_string() {
        let maze_str = "
#####
#   #
#=#=#
#...#
#####
";
        let maze = Maze::from_string(maze_str);
        assert!(maze.is_ok());
        let maze = maze.unwrap();
        pretty_assertions::assert_eq!(maze.width, 5);
        pretty_assertions::assert_eq!(maze.height, 5);
        pretty_assertions::assert_eq!(maze.get_tile(0, 0), Some(Tile::Wall));
        pretty_assertions::assert_eq!(maze.get_tile(1, 1), Some(Tile::Path));
        pretty_assertions::assert_eq!(maze.get_tile(2, 2), Some(Tile::Wall));
    }

    #[test]
    fn test_maze_invalid_size() {
        let maze_str = "
#####
####
# # #
";
        let maze = Maze::from_string(maze_str);
        assert!(maze.is_err());
        pretty_assertions::assert_eq!(
            maze.err().unwrap(),
            "Inconsistent line length: line 1 has length 5, line 2 has length 4"
        );
    }

    #[test]
    fn test_maze_invalid_character() {
        let maze_str = "
#####
#   #
# # #
#   #
#@###
";
        let maze = Maze::from_string(maze_str);
        assert!(maze.is_err());
        pretty_assertions::assert_eq!(maze.err().unwrap(), "Unknown tile character '@' at (1, 4)");
    }

    #[test]
    fn test_maze_to_string() {
        let maze_str = "
#####
#   #
#=#=#
#...#
#####
";
        let maze = Maze::from_string(maze_str).unwrap();
        let maze_display = maze.to_string();
        pretty_assertions::assert_eq!(maze_display.trim(), maze_str.trim());
    }
}
