#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Path,
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
        println!("{}", s);
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

    pub fn iter(&self) -> MazeIterator {
        MazeIterator {
            maze: self,
            current: 0,
        }
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
# # #
#   #
#####
";
        let maze = Maze::from_string(maze_str);
        assert!(maze.is_ok());
        let maze = maze.unwrap();
        assert_eq!(maze.width, 5);
        assert_eq!(maze.height, 5);
        assert_eq!(maze.get_tile(0, 0), Some(Tile::Wall));
        assert_eq!(maze.get_tile(1, 1), Some(Tile::Path));
        assert_eq!(maze.get_tile(2, 2), Some(Tile::Wall));
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
        assert_eq!(
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
        assert_eq!(maze.err().unwrap(), "Unknown tile character '@' at (1, 4)");
    }
}
