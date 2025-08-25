use crate::maze;
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Still,
    Up,
    Down,
    Left,
    Right,
}

pub struct Munch {
    pub x: usize,
    pub y: usize,
}

impl Munch {
    pub fn new(x: usize, y: usize) -> Self {
        Munch { x, y }
    }

    pub fn walk(&mut self, direction: Direction, maze: &maze::Maze) {
        match direction {
            Direction::Up => {
                if !maze.is_wall(self.x, self.y + maze.height - 1) {
                    if self.y == 0 {
                        self.y = maze.height - 1;
                    } else {
                        self.y -= 1;
                    }
                }
            }
            Direction::Down => {
                if !maze.is_wall(self.x, self.y + 1) {
                    if self.y == maze.height - 1 {
                        self.y = 0;
                    } else {
                        self.y += 1;
                    }
                }
            }
            Direction::Left => {
                if !maze.is_wall(self.x + maze.width - 1, self.y) {
                    if self.x == 0 {
                        self.x = maze.width - 1;
                    } else {
                        self.x -= 1;
                    }
                }
            }
            Direction::Right => {
                if !maze.is_wall(self.x + 1, self.y) {
                    if self.x == maze.width - 1 {
                        self.x = 0;
                    } else {
                        self.x += 1;
                    }
                }
            }
            _ => {}
        }
    }
}
