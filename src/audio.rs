use ggez::audio;
use ggez::audio::SoundSource;
use ggez::Context;

pub struct Audio {
    chomp_sound: audio::Source,
    power_pellet_sound: audio::Source,
    death_sound: audio::Source,
    beginning_sound: audio::Source,
    eat_ghost_sound: audio::Source,
}

macro_rules! audio_source {
    ($ctx:expr, $path:expr) => {
        audio::Source::from_data($ctx, audio::SoundData::from_bytes(include_bytes!($path))).unwrap()
    };
}

impl Audio {
    pub fn new(ctx: &mut Context) -> Audio {
        Audio {
            chomp_sound: audio_source!(ctx, "../resources/chomp.wav"),
            power_pellet_sound: audio_source!(ctx, "../resources/eatpowerpellet.wav"),
            death_sound: audio_source!(ctx, "../resources/death.wav"),
            beginning_sound: audio_source!(ctx, "../resources/beginning.wav"),
            eat_ghost_sound: audio_source!(ctx, "../resources/eatghost.wav"),
        }
    }

    pub fn start_chomp(&mut self, ctx: &Context) {
        self.chomp_sound.set_repeat(true);
        self.play_chomp(ctx);
    }

    fn play_chomp(&mut self, ctx: &Context) {
        match self.chomp_sound.play(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error playing chomp sound: {}", err),
        }
    }

    pub fn play_power_pellet(&mut self, ctx: &Context) {
        match self.power_pellet_sound.play(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error playing power pellet sound: {}", err),
        }
    }

    pub fn play_death(&mut self, ctx: &Context) {
        match self.chomp_sound.stop(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error stopping chomp sound: {}", err),
        }
        match self.death_sound.play(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error playing death sound: {}", err),
        }
    }

    pub fn death_is_finished(&self) -> bool {
        !self.death_sound.playing()
    }

    pub fn play_beginning(&mut self, ctx: &Context) {
        match self.beginning_sound.play(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error playing beginning sound: {}", err),
        }
    }

    pub fn beginning_is_finished(&self) -> bool {
        !self.beginning_sound.playing()
    }

    pub fn play_eat_ghost(&mut self, ctx: &Context) {
        match self.eat_ghost_sound.play(ctx) {
            Ok(_) => {}
            Err(err) => eprintln!("Error playing eat ghost sound: {}", err),
        }
    }
}
