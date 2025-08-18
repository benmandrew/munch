pub struct Window {
    pub terminal: pancurses::Window,
    pub game_area: pancurses::Window,
}

const GAME_AREA_HEIGHT: i32 = 22;
const GAME_AREA_WIDTH: i32 = 21 * 2; // Width is doubled for better aspect ratio

impl Window {
    pub fn new(terminal: pancurses::Window) -> Self {
        let (height, width) = terminal.get_max_yx();
        match terminal.subwin(
            GAME_AREA_HEIGHT,
            GAME_AREA_WIDTH,
            (height - GAME_AREA_HEIGHT) / 2,
            (width - GAME_AREA_WIDTH) / 2,
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

    pub fn draw_game(&self) {
        self.game_area.clear();
        self.game_area.draw_box(0, 0);
        self.game_area.refresh();
    }

    pub fn refresh(&self) {
        self.terminal.refresh();
        self.game_area.refresh();
    }

    pub fn get_input(&self) -> Option<pancurses::Input> {
        self.terminal.getch()
    }
}
