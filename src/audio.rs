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

impl Audio {
    pub fn new(ctx: &mut Context) -> Audio {
        Audio {
            chomp_sound: audio::Source::new(ctx, "/chomp.wav").unwrap(),
            power_pellet_sound: audio::Source::new(ctx, "/eatpowerpellet.wav").unwrap(),
            death_sound: audio::Source::new(ctx, "/death.wav").unwrap(),
            beginning_sound: audio::Source::new(ctx, "/beginning.wav").unwrap(),
            eat_ghost_sound: audio::Source::new(ctx, "/eatghost.wav").unwrap(),
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
