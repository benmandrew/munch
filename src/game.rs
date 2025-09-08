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
    death_in_progress: bool,
    startup_in_progress: bool,
}

impl Game {
    pub fn new(ctx: &mut Context, config: config::Config) -> Game {
        let window = window::Window::new(ctx);
        let mut audio = audio::Audio::new(ctx);
        let game_logic = game_logic::GameLogic::new(config);
        let spin_sleep = spin_sleep::SpinSleeper::new(100_000)
            .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);
        audio.play_beginning(ctx);
        Game {
            window,
            audio,
            spin_sleep,
            last_game_update: std::time::Instant::now(),
            game_logic,
            death_in_progress: false,
            startup_in_progress: true,
        }
    }

    fn sleep_frame(&mut self) {
        self.spin_sleep.sleep_until(
            self.last_game_update + std::time::Duration::from_millis(FRAME_TIME as u64),
        );
    }

    fn get_time_delta(&self) -> f32 {
        self.last_game_update.elapsed().as_millis() as f32 / 1000.0
    }

    fn update_death(&mut self, ctx: &mut Context) -> GameResult {
        if self.death_in_progress && self.audio.death_is_finished() {
            // self.death_in_progress = false;
            std::process::exit(0);
        }
        if self.death_in_progress {
            return Ok(());
        }
        self.death_in_progress = true;
        self.window.reset_frame();
        self.audio.play_death(ctx);
        Ok(())
    }

    fn update_startup(&mut self, ctx: &mut Context) -> GameResult {
        if self.startup_in_progress && self.audio.beginning_is_finished() {
            self.audio.start_chomp(ctx);
            self.startup_in_progress = false;
        }
        Ok(())
    }

    fn handle_audio_triggers(&mut self, ctx: &mut Context, rs: &game_logic::ReturnState) {
        if rs.eaten_power_pellet {
            self.audio.play_power_pellet(ctx);
        }
        if rs.eaten_ghost {
            self.audio.play_eat_ghost(ctx);
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.sleep_frame();
        let result = if self.startup_in_progress {
            self.update_startup(ctx)
        } else if self.game_logic.munch_is_dead {
            self.update_death(ctx)
        } else {
            let time_delta = self.get_time_delta();
            let rs = self.game_logic.update(time_delta);
            self.handle_audio_triggers(ctx, &rs);
            Ok(())
        };
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
            self.death_in_progress,
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
