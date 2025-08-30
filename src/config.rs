use crate::maze;

pub struct Config {
    pub maze: maze::Maze,
    pub player_pos: Option<(usize, usize)>,
    pub ghosts_pos: Vec<(usize, usize)>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(file_path) {
            Ok(contents) => Self::from_string(&contents),
            Err(e) => Err(format!("Failed to read config file: {}: {}", file_path, e)),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let lines: Vec<&str> = s.trim().split('\n').collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut maze = Vec::with_capacity(width * height);
        let mut player_pos = None;
        let mut ghosts_pos = Vec::new();
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
                    '#' => maze.push(maze::Tile::Wall),
                    ' ' => maze.push(maze::Tile::Path),
                    '=' => maze.push(maze::Tile::PlayerImpassable),
                    '.' => maze.push(maze::Tile::Dot),
                    'P' => {
                        assert!(player_pos.is_none(), "Multiple player positions found");
                        player_pos = Some((x, y));
                        maze.push(maze::Tile::Path)
                    }
                    'G' => {
                        ghosts_pos.push((x, y));
                        maze.push(maze::Tile::Path)
                    }
                    _ => {
                        return Err(format!("Unknown tile character '{}' at ({}, {})", c, x, y));
                    }
                }
            }
        }
        Ok(Config {
            maze: maze::Maze::new(width, height, maze),
            player_pos,
            ghosts_pos,
        })
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Config {
            maze: maze::Maze::empty(),
            player_pos: None,
            ghosts_pos: Vec::new(),
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
#GP #
#=#=#
#...#
#####
";
        let config_result = Config::from_string(maze_str);
        assert!(config_result.is_ok());
        let config = config_result.unwrap();

        pretty_assertions::assert_eq!(config.maze.width, 5);
        pretty_assertions::assert_eq!(config.maze.height, 5);
        pretty_assertions::assert_eq!(config.maze.get_tile(0, 0), Some(maze::Tile::Wall));
        pretty_assertions::assert_eq!(config.maze.get_tile(1, 1), Some(maze::Tile::Path));
        pretty_assertions::assert_eq!(config.maze.get_tile(4, 4), Some(maze::Tile::Wall));
        pretty_assertions::assert_eq!(
            config.maze.get_tile(1, 2),
            Some(maze::Tile::PlayerImpassable)
        );
        pretty_assertions::assert_eq!(config.player_pos, Some((2, 1)));
        pretty_assertions::assert_eq!(config.ghosts_pos, vec![(1, 1)]);
    }

    #[test]
    fn test_maze_invalid_size() {
        let maze_str = "
#####
####
# # #
";
        let config = Config::from_string(maze_str);
        assert!(config.is_err());
        pretty_assertions::assert_eq!(
            config.err().unwrap(),
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
        let config = Config::from_string(maze_str);
        assert!(config.is_err());
        pretty_assertions::assert_eq!(
            config.err().unwrap(),
            "Unknown tile character '@' at (1, 4)"
        );
    }
}
