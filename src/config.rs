use crate::{ghost, maze};

pub struct Config {
    pub maze: maze::Maze,
    pub player_pos: Option<(usize, usize)>,
    pub ghosts_pos: Vec<(usize, usize, ghost::Personality)>,
}

fn match_maze_char(c: char) -> Result<maze::Tile, String> {
    match c {
        '#' => Ok(maze::Tile::Wall),
        ' ' | 'M' => Ok(maze::Tile::Path),
        '=' | 'B' | 'P' | 'I' | 'C' => Ok(maze::Tile::PlayerImpassable),
        '.' => Ok(maze::Tile::Dot),
        _ => Err(format!("Unknown tile character '{}'", c)),
    }
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
                match match_maze_char(c) {
                    Ok(tile) => maze.push(tile),
                    Err(e) => return Err(format!("Error at ({}, {}): {}", x, y, e)),
                };
                match c {
                    'M' => {
                        assert!(player_pos.is_none(), "Multiple player positions found");
                        player_pos = Some((x, y));
                    }
                    'B' => {
                        ghosts_pos.push((x, y, ghost::Personality::Blinky));
                    }
                    'I' => {
                        ghosts_pos.push((x, y, ghost::Personality::Inky));
                    }
                    'P' => {
                        ghosts_pos.push((x, y, ghost::Personality::Pinky));
                    }
                    'C' => {
                        ghosts_pos.push((x, y, ghost::Personality::Clyde));
                    }
                    _ => {}
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
    use crate::ghost::Personality;

    use super::*;

    #[test]
    fn test_maze_from_string() {
        let maze_str = "
#####
#PM #
#=#=#
#.BI#
#####
";
        let config_result = Config::from_string(maze_str);
        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        pretty_assertions::assert_eq!(config.maze.width, 5);
        pretty_assertions::assert_eq!(config.maze.height, 5);
        pretty_assertions::assert_eq!(config.maze.get_tile(0, 0), Some(maze::Tile::Wall));
        pretty_assertions::assert_eq!(
            config.maze.get_tile(1, 1),
            Some(maze::Tile::PlayerImpassable)
        );
        pretty_assertions::assert_eq!(config.maze.get_tile(4, 4), Some(maze::Tile::Wall));
        pretty_assertions::assert_eq!(
            config.maze.get_tile(1, 2),
            Some(maze::Tile::PlayerImpassable)
        );
        pretty_assertions::assert_eq!(config.player_pos, Some((2, 1)));
        pretty_assertions::assert_eq!(
            config.ghosts_pos,
            vec![
                (1, 1, Personality::Pinky),
                (2, 3, Personality::Blinky),
                (3, 3, Personality::Inky)
            ]
        );
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
            "Error at (1, 4): Unknown tile character '@'"
        );
    }
}
