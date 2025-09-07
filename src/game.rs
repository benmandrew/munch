use ggez::error::GameError;
use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

use crate::{audio, config, game_logic, window};

const FRAME_TIME: f32 = 1000.0 / 120.0;

pub struct Game {
    window: window::Window,
    audio: audio::Audio,
    spin_sleep: spin_sleep::SpinSleeper,
    last_game_update: std::time::Instant,
    game_logic: game_logic::GameLogic,
}

impl Game {
    pub fn new(ctx: &mut Context, config: config::Config) -> Game {
        let window = window::Window::new(ctx);
        let mut audio = audio::Audio::new(ctx);
        let game_logic = game_logic::GameLogic::new(config);
        let spin_sleep = spin_sleep::SpinSleeper::new(100_000)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);
        audio.start_chomp(ctx);
        Game {
            window,
            audio,
            spin_sleep,
            last_game_update: std::time::Instant::now(),
            game_logic,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.spin_sleep.sleep_until(
            self.last_game_update + std::time::Duration::from_millis(FRAME_TIME as u64),
        );
        let time_delta = self.last_game_update.elapsed().as_millis() as f32 / 1000.0;
        let result = self.game_logic.update(time_delta);
        self.last_game_update = std::time::Instant::now();
        result
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        let keycode = match input.keycode {
            Some(key) => key,
            None => return Ok(()),
        };
        self.game_logic.handle_movement(keycode);
        match keycode {
            KeyCode::Escape | KeyCode::Q => {
                ctx.request_quit();
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.window.draw(
            ctx,
            &self.game_logic.maze,
            &self.game_logic.munch,
            &self.game_logic.ghosts,
            self.game_logic.score,
        )
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        width: f32,
        height: f32,
    ) -> Result<(), GameError> {
        self.window.resize(width, height);
        Ok(())
    }
}
