extern crate pancurses;

pub mod window;

fn pancurses_setup() -> pancurses::Window {
    let terminal = pancurses::initscr();
    pancurses::curs_set(0);
    terminal.keypad(true);
    terminal.nodelay(true);
    terminal
}

fn main() {
    let terminal = pancurses_setup();
    let window = window::Window::new(terminal);
    window.draw_game();
    window.refresh();

    loop {
        match window.get_input() {
            Some(pancurses::Input::KeyResize) => {
                pancurses::resize_term(0, 0);
            }
            Some(pancurses::Input::Character('q')) => break,
            _ => (),
        }
    }
    pancurses::endwin();
}
