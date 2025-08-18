extern crate pancurses;

mod maze;
mod window;

struct CursesGuard;

impl Drop for CursesGuard {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

fn pancurses_setup() -> pancurses::Window {
    let terminal = pancurses::initscr();
    pancurses::curs_set(0);
    terminal.keypad(true);
    terminal.nodelay(true);
    terminal
}

fn main() {
    let _guard = CursesGuard;
    let terminal = pancurses_setup();
    let maze = match maze::Maze::from_file("resources/maze.txt") {
        Ok(maze) => maze,
        Err(e) => {
            eprintln!("Error loading maze: {}", e);
            return;
        }
    };
    let mut window = window::Window::new(terminal, maze.height, maze.width);
    window.draw_game(&maze);
    window.refresh();
    loop {
        match window.get_input() {
            Some(pancurses::Input::KeyResize) => {
                pancurses::resize_term(0, 0);
                window.resize(maze.height, maze.width);
                window.draw_game(&maze);
            }
            Some(pancurses::Input::Character('q')) => break,
            _ => (),
        }
    }
    pancurses::endwin();
}
