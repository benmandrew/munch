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
    progress_to_next_square: f32,
    move_direction: Direction,
}

impl Munch {
    pub fn new(x: usize, y: usize) -> Self {
        Munch {
            x,
            y,
            progress_to_next_square: 0.0,
            move_direction: Direction::Still,
        }
    }

    pub fn get_draw_pos(&self) -> (f32, f32) {
        let x = self.x as f32;
        let y = self.y as f32;
        let (x, y) = match self.move_direction {
            Direction::Up => (x, y - self.progress_to_next_square),
            Direction::Down => (x, y + self.progress_to_next_square),
            Direction::Left => (x - self.progress_to_next_square, y),
            Direction::Right => (x + self.progress_to_next_square, y),
            Direction::Still => (x, y),
        };
        (x, y)
    }

    pub fn walk(&mut self, direction: Direction, maze: &maze::Maze, time_delta: f32) {
        let offset = 3.0 * time_delta;
        if self.move_direction == Direction::Still {
            self.move_direction = direction;
        }
        match (self.move_direction, direction) {
            // Moving up
            (Direction::Up, Direction::Up) => {
                if !maze.is_wall(self.x, self.y + maze.height - 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Down, Direction::Up) => {
                // Allow for reversing direction close to a wall
                if !maze.is_wall(self.x, self.y + maze.height - 1)
                    || self.progress_to_next_square >= offset * 2.0
                {
                    self.y += 1;
                    self.progress_to_next_square =
                        1.0 - self.progress_to_next_square + offset * 2.0;
                    self.move_direction = Direction::Up;
                }
            }
            (Direction::Right, Direction::Up) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x, self.y + maze.height - 1)
                    && self.progress_to_next_square < 0.1
                {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Up;
                } else if !maze.is_wall(self.x + 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Left, Direction::Up) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x, self.y + maze.height - 1)
                    && self.progress_to_next_square < 0.1
                {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Up;
                } else if !maze.is_wall(self.x - 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            // Moving right
            (Direction::Up, Direction::Right) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x + 1, self.y) && self.progress_to_next_square < 0.1 {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Right;
                } else if !maze.is_wall(self.x, self.y + maze.height - 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Down, Direction::Right) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x + 1, self.y) && self.progress_to_next_square < 0.1 {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Right;
                } else if !maze.is_wall(self.x, self.y + 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Right, Direction::Right) => {
                if !maze.is_wall(self.x + 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Left, Direction::Right) => {
                // Allow for reversing direction close to a wall
                if !maze.is_wall(self.x + 1, self.y) || self.progress_to_next_square >= offset * 2.0
                {
                    self.x -= 1;
                    self.progress_to_next_square =
                        1.0 - self.progress_to_next_square + offset * 2.0;
                    self.move_direction = Direction::Right;
                }
            }
            // Moving down
            (Direction::Up, Direction::Down) => {
                // Allow for reversing direction close to a wall
                if !maze.is_wall(self.x, self.y + 1) || self.progress_to_next_square >= offset * 2.0
                {
                    self.y -= 1;
                    self.progress_to_next_square =
                        1.0 - self.progress_to_next_square + offset * 2.0;
                    self.move_direction = Direction::Down;
                }
            }
            (Direction::Down, Direction::Down) => {
                if !maze.is_wall(self.x, self.y + 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Right, Direction::Down) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x, self.y + 1) && self.progress_to_next_square < 0.1 {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Down;
                } else if !maze.is_wall(self.x + 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Left, Direction::Down) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x, self.y + 1) && self.progress_to_next_square < 0.1 {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Down;
                } else if !maze.is_wall(self.x + maze.width - 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            // Moving left
            (Direction::Up, Direction::Left) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x + maze.width - 1, self.y)
                    && self.progress_to_next_square < 0.1
                {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Left;
                } else if !maze.is_wall(self.x, self.y + maze.height - 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Down, Direction::Left) => {
                // Prevent jumping around corners we've already passed
                if !maze.is_wall(self.x + maze.width - 1, self.y)
                    && self.progress_to_next_square < 0.1
                {
                    self.progress_to_next_square = 0.0;
                    self.move_direction = Direction::Left;
                } else if !maze.is_wall(self.x, self.y + 1) {
                    self.progress_to_next_square += offset;
                }
            }
            (Direction::Right, Direction::Left) => {
                // Allow for reversing direction close to a wall
                if !maze.is_wall(self.x + maze.width - 1, self.y)
                    || self.progress_to_next_square >= offset * 2.0
                {
                    self.x += 1;
                    self.progress_to_next_square =
                        1.0 - self.progress_to_next_square + offset * 2.0;
                    self.move_direction = Direction::Left;
                }
            }
            (Direction::Left, Direction::Left) => {
                if !maze.is_wall(self.x + maze.width - 1, self.y) {
                    self.progress_to_next_square += offset;
                }
            }
            _ => {}
        }
        self.update_discrete_position(maze);
    }

    fn update_discrete_position(&mut self, maze: &maze::Maze) {
        if self.progress_to_next_square < 1.0 {
            return;
        }
        self.progress_to_next_square = 0.0;
        match self.move_direction {
            Direction::Up => {
                if self.y == 0 {
                    self.y = maze.height - 1;
                } else {
                    self.y -= 1;
                }
            }
            Direction::Down => {
                if self.y == maze.height - 1 {
                    self.y = 0;
                } else {
                    self.y += 1;
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    self.x = maze.width - 1;
                } else {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                if self.x == maze.width - 1 {
                    self.x = 0;
                } else {
                    self.x += 1;
                }
            }
            _ => {}
        }
    }
}
