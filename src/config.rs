use crate::{ghost, maze};

pub const TILE_SIZE: f32 = 48.0;

pub struct Config {
    pub maze: maze::Maze,
    pub player_pos: Option<(i32, i32)>,
    pub ghosts_pos: Vec<(i32, i32, ghost::Personality)>,
}

fn match_maze_char(c: char) -> Result<maze::Tile, String> {
    match c {
        '#' => Ok(maze::Tile::Wall),
        ' ' | 'M' => Ok(maze::Tile::Path),
        '=' | 'B' | 'P' | 'I' | 'C' => Ok(maze::Tile::PlayerImpassable),
        'R' => Ok(maze::Tile::Respawn),
        '.' => Ok(maze::Tile::Dot),
        '*' => Ok(maze::Tile::PowerPellet),
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

    fn add_tile(
        maze: &mut Vec<maze::Tile>,
        c: char,
        respawn_point: &mut Option<(i32, i32)>,
        x: i32,
        y: i32,
    ) -> Result<(), String> {
        match match_maze_char(c) {
            Ok(tile) => {
                if tile == maze::Tile::Respawn {
                    if respawn_point.is_some() {
                        return Err("Second respawn tile found".to_string());
                    }
                    respawn_point.replace((x, y));
                }
                maze.push(tile);
            }
            Err(e) => return Err(e),
        };
        Ok(())
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let lines: Vec<&str> = s.trim().split('\n').collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut maze = Vec::with_capacity(width * height);
        let mut player_pos = None;
        let mut ghosts_pos = Vec::new();
        let mut respawn_point: Option<(i32, i32)> = None;
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
                match Self::add_tile(&mut maze, c, &mut respawn_point, x as i32, y as i32) {
                    Ok(()) => {}
                    Err(e) => return Err(format!("Error at ({}, {}): {}", x, y, e)),
                };
                match c {
                    'M' => {
                        assert!(player_pos.is_none(), "Multiple player positions found");
                        player_pos = Some((x as i32, y as i32));
                    }
                    'B' => {
                        ghosts_pos.push((x as i32, y as i32, ghost::Personality::Blinky));
                    }
                    'I' => {
                        ghosts_pos.push((x as i32, y as i32, ghost::Personality::Inky));
                    }
                    'P' => {
                        ghosts_pos.push((x as i32, y as i32, ghost::Personality::Pinky));
                    }
                    'C' => {
                        ghosts_pos.push((x as i32, y as i32, ghost::Personality::Clyde));
                    }
                    _ => {}
                }
            }
        }
        match respawn_point {
            Some(respawn_point) => Ok(Config {
                maze: maze::Maze::new(width as i32, height as i32, maze, respawn_point),
                player_pos,
                ghosts_pos,
            }),
            None => Err("No respawn tile found".to_string()),
        }
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
######
#PM  #
#=#*R#
#.BI #
######
";
        let config_result = Config::from_string(maze_str);
        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        pretty_assertions::assert_eq!(config.maze.width, 6);
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
#  *#
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
