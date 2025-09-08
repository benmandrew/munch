use crate::actor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Path,
    PlayerImpassable,
    Respawn,
    Dot,
    PowerPellet,
}

pub fn player_passable(tile: &Tile) -> bool {
    matches!(tile, Tile::Path | Tile::Dot | Tile::PowerPellet)
}

pub fn ghost_passable(tile: &Tile) -> bool {
    matches!(
        tile,
        Tile::Path | Tile::PlayerImpassable | Tile::Dot | Tile::PowerPellet | Tile::Respawn
    )
}

#[derive(Debug)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    maze: Vec<Tile>,
    pub respawn_point: (i32, i32),
    pub n_dots: i32,
}

impl Maze {
    pub fn new(width: i32, height: i32, maze: Vec<Tile>, respawn_point: (i32, i32)) -> Self {
        let n_dots = maze.iter().filter(|&&t| t == Tile::Dot).count() as i32;
        Maze {
            width,
            height,
            maze,
            respawn_point,
            n_dots,
        }
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Maze {
            width: 0,
            height: 0,
            maze: Vec::new(),
            respawn_point: (0, 0),
            n_dots: 0,
        }
    }

    pub fn iter(&self) -> MazeIterator<'_> {
        MazeIterator {
            maze: self,
            current: 0,
        }
    }

    fn index(&self, mut x: i32, mut y: i32) -> usize {
        while x < 0 {
            x += self.width;
        }
        while y < 0 {
            y += self.height;
        }
        ((y % self.height) * self.width + (x % self.width)) as usize
    }

    pub fn is_player_passable(&self, x: i32, y: i32) -> bool {
        player_passable(self.maze.get(self.index(x, y)).unwrap())
    }

    pub fn is_ghost_passable(&self, x: i32, y: i32) -> bool {
        ghost_passable(self.maze.get(self.index(x, y)).unwrap())
    }

    pub fn eat_dots(&mut self, munch: &actor::Actor) -> i32 {
        let covering_tiles = munch.get_covering_tiles(0.45);
        let mut eaten = 0;
        for (x, y) in covering_tiles {
            if self.maze.get(self.index(x, y)) == Some(&Tile::Dot) {
                eaten += 1;
                let index = self.index(x, y);
                self.maze[index] = Tile::Path;
            }
        }
        self.n_dots -= eaten;
        eaten
    }

    pub fn eat_power_pellets(&mut self, munch: &actor::Actor) -> i32 {
        let covering_tiles = munch.get_covering_tiles(0.45);
        let mut eaten = 0;
        for (x, y) in covering_tiles {
            if self.maze.get(self.index(x, y)) == Some(&Tile::PowerPellet) {
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
        let mut line = String::with_capacity(self.width as usize);
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
                    Tile::Respawn => 'R',
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
    current: i32,
}

impl<'a> Iterator for MazeIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.maze.maze.len() as i32 {
            let tile = &self.maze.maze[self.current as usize];
            self.current += 1;
            Some(tile)
        } else {
            None
        }
    }
}

#[cfg(test)]
impl Maze {
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.maze[(y * self.width + x) as usize])
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
# R #
#=#=#
#..*#
#####
";
        let maze = config::Config::from_string(maze_str).unwrap().maze;
        let maze_display = maze.to_string();
        pretty_assertions::assert_eq!(maze_display.trim(), maze_str.trim());
    }
}
