use ggez::conf;
use ggez::event::{self, EventLoop};
use ggez::{Context, ContextBuilder};

mod actor;
mod game;
mod game_logic;
mod ghost;
mod maze;
mod window;

fn init_logger() {
    let mut builder = colog::basic_builder();
    builder.filter_level(log::LevelFilter::Warn);
    builder.init();
}

fn init_context() -> (Context, EventLoop<()>) {
    let resources_dir = std::path::PathBuf::from("./resources");
    let window_mode = conf::WindowMode::default()
        .dimensions(1600.0, 1200.0)
        .resizable(true);
    let window_setup = conf::WindowSetup::default().vsync(false).title("Munch");
    ContextBuilder::new("Munch", "Ben M. Andrew")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resources_dir)
        .build()
        .expect("Could not create ggez context")
}

fn main() {
    init_logger();
    let (mut ctx, event_loop) = init_context();
    let game = game::Game::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
