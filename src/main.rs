extern crate pancurses;

fn main() {
    let window = pancurses::initscr();

    window.keypad(true);

    window.printw("Press q to exit\n");
    window.refresh();

    loop {
        match window.getch() {
            Some(pancurses::Input::KeyResize) => {
                pancurses::resize_term(0, 0);
                let (y, x) = window.get_max_yx();
                window.mvaddstr(y - 1, x - 8, "Resized");
            }
            Some(pancurses::Input::Character('q')) => break,
            _ => (),
        }
    }
    pancurses::endwin();
}
