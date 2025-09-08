use clap::Parser;
use ggez::conf;
use ggez::event::{self, EventLoop};
use ggez::{Context, ContextBuilder};

mod actor;
mod audio;
mod config;
mod game;
mod game_logic;
mod ghost;
mod maze;
mod spritesheet;
mod window;

fn init_logger(log_level: log::LevelFilter) {
    let mut builder = colog::basic_builder();
    builder.filter_level(log_level);
    builder.init();
}

/// A simple program to greet someone
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Reads version from Cargo.toml, uses doc comment for about
struct Cli {
    #[arg(short, long, default_value_t = log::LevelFilter::Warn)]
    log_level: log::LevelFilter,
}

fn init_context() -> (Context, EventLoop<()>) {
    let window_mode = conf::WindowMode::default()
        .dimensions(1600.0, 1200.0)
        .resizable(true);
    let window_setup = conf::WindowSetup::default().vsync(false).title("Munch");
    ContextBuilder::new("Munch", "Ben M. Andrew")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context")
}

fn init_config() -> config::Config {
    // match config::Config::from_file("resources/maze.txt") {
    match config::Config::from_string(include_str!("../resources/maze.txt")) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error loading config: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    init_logger(cli.log_level);
    let (mut ctx, event_loop) = init_context();
    let config = init_config();
    let game = game::Game::new(&mut ctx, config);
    event::run(ctx, event_loop, game);
}
