use crate::actor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Path,
    PlayerImpassable,
    Dot,
    PowerPellet,
}

pub fn player_passable(tile: &Tile) -> bool {
    matches!(tile, Tile::Path | Tile::Dot | Tile::PowerPellet)
}

pub fn ghost_passable(tile: &Tile) -> bool {
    matches!(
        tile,
        Tile::Path | Tile::PlayerImpassable | Tile::Dot | Tile::PowerPellet
    )
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
                    Tile::PowerPellet => '*',
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
#..*#
#####
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        let maze_display = maze.to_string();
        pretty_assertions::assert_eq!(maze_display.trim(), maze_str.trim());
    }
}
