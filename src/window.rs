use pancurses::chtype;

use crate::maze;

pub struct Window {
    pub terminal: pancurses::Window,
    pub game_area: pancurses::Window,
}

fn maze_to_window_idx(maze_y: usize, maze_x: usize) -> (usize, usize) {
    (maze_y, maze_x * 2)
}

impl Window {
    pub fn new(terminal: pancurses::Window, maze_height: usize, maze_width: usize) -> Self {
        let (height, width) = terminal.get_max_yx();
        let (game_area_height, game_area_width) = maze_to_window_idx(maze_height, maze_width);
        match terminal.subwin(
            game_area_height as i32,
            game_area_width as i32,
            (height - game_area_height as i32) / 2,
            (width - game_area_width as i32) / 2,
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

    pub fn resize(&mut self, maze_height: usize, maze_width: usize) {
        let (height, width) = self.terminal.get_max_yx();
        let (game_area_height, game_area_width) = maze_to_window_idx(maze_height, maze_width);
        self.game_area.mvderwin(
            (height - game_area_height as i32) / 2,
            (width - game_area_width as i32) / 2,
        );
        self.game_area
            .resize(game_area_height as i32, game_area_width as i32);
    }

    pub fn draw_game(&self, maze: &maze::Maze) {
        self.terminal.clear();
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
            let (y, x) = maze_to_window_idx(y, x);
            self.game_area.mv(y as i32, x as i32);
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
