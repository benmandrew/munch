use pancurses::chtype;

use crate::maze;

pub struct Window {
    pub terminal: pancurses::Window,
    pub game_area: pancurses::Window,
}

// const GAME_AREA_HEIGHT: i32 = 22;
// const GAME_AREA_WIDTH: i32 = 21 * 2; // Width is doubled for better aspect ratio

// Calculate the dimensions of the game area based on the maze size
fn game_area_dims(maze_height: i32, maze_width: i32) -> (i32, i32) {
    (maze_height, maze_width * 2)
}

impl Window {
    pub fn new(terminal: pancurses::Window, maze_height: i32, maze_width: i32) -> Self {
        let (height, width) = terminal.get_max_yx();
        let (game_area_height, game_area_width) = game_area_dims(maze_height, maze_width);
        match terminal.subwin(
            game_area_height,
            game_area_width,
            (height - game_area_height) / 2,
            (width - game_area_width) / 2,
        ) {
            Ok(game_area) => Window {
                terminal,
                game_area,
            },
            Err(_) => {
                pancurses::endwin();
                panic!("Failed to create game area");
            }
        }
    }

    pub fn draw_game(&self, maze: &maze::Maze) {
        self.game_area.clear();
        self.draw_maze(maze);
        self.game_area.refresh();
    }

    fn draw_maze(&self, maze: &maze::Maze) {
        for (i, row) in maze.iter().enumerate() {
            let y = i / maze.width;
            let x = i % maze.width;
            let ch = match row {
                maze::Tile::Wall => pancurses::ACS_BOARD() as chtype,
                maze::Tile::Path => ' ' as chtype,
            };
            self.game_area.mv(y as i32, x as i32 * 2);
            self.game_area.addch(ch);
            self.game_area.addch(ch);
        }
    }

    pub fn refresh(&self) {
        self.terminal.refresh();
        self.game_area.refresh();
    }

    pub fn get_input(&self) -> Option<pancurses::Input> {
        self.terminal.getch()
    }
}
