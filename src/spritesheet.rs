use ggez::glam;
use ggez::graphics;
use ggez::Context;

use crate::{actor, config, ghost};

pub struct SpriteSheet {
    pub image: graphics::Image,
    sprite_u_width: f32,
    sprite_v_height: f32,
}

const SPRITESHEET_PATH: &str = "/spritesheet.png";
const SPRITE_WIDTH: u32 = 11;
const SPRITE_HEIGHT: u32 = 11;
const ANIM_FPS: usize = 60 / 6; // 6 frames per second
const DEATH_ANIM_FPS: usize = 60 / 3; // 3 frames per second

impl SpriteSheet {
    pub fn new(ctx: &mut Context) -> Self {
        let image = match graphics::Image::from_path(ctx, SPRITESHEET_PATH) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to load spritesheet image: {}", e);
                std::process::exit(1);
            }
        };
        let rows = (image.height() / SPRITE_HEIGHT) as usize;
        let columns = (image.width() / SPRITE_WIDTH) as usize;
        let sprite_u_width = 1.0 / columns as f32;
        let sprite_v_height = 1.0 / rows as f32;
        SpriteSheet {
            image,
            sprite_u_width,
            sprite_v_height,
        }
    }

    fn sprite_pos(&self, i: u32, j: u32) -> (f32, f32) {
        (
            i as f32 * self.sprite_u_width,
            j as f32 * self.sprite_v_height,
        )
    }

    fn draw_sprite(&self, canvas: &mut graphics::Canvas, i: u32, j: u32, dest: glam::Vec2) {
        let (u, v) = self.sprite_pos(i, j);
        // Make src_rect slightly smaller to avoid texture bleeding
        let src_rect = graphics::Rect::new(
            u,
            v,
            self.sprite_u_width - 0.002,
            self.sprite_v_height - 0.002,
        );
        let dest_scale = glam::Vec2::new(
            (config::TILE_SIZE - 4.0) / SPRITE_WIDTH as f32,
            (config::TILE_SIZE - 4.0) / SPRITE_HEIGHT as f32,
        );
        let adapted_dest = glam::Vec2::new(dest.x + 2.0, dest.y + 2.0);
        canvas.draw(
            &self.image,
            graphics::DrawParam::new()
                .dest(adapted_dest)
                .scale(dest_scale)
                .src(src_rect),
        );
    }

    pub fn draw_munch(
        &self,
        canvas: &mut graphics::Canvas,
        direction: actor::Direction,
        pos: glam::Vec2,
        frame: usize,
    ) {
        let anim_frame = ((frame / ANIM_FPS) % 3) as u32;
        if anim_frame == 2 {
            self.draw_sprite(canvas, 0, 6, pos);
        }
        match direction {
            actor::Direction::Still | actor::Direction::Right => {
                self.draw_sprite(canvas, 1 + anim_frame, 6, pos)
            }
            actor::Direction::Down => self.draw_sprite(canvas, 3 + anim_frame, 6, pos),
            actor::Direction::Left => self.draw_sprite(canvas, 5 + anim_frame, 6, pos),
            actor::Direction::Up => self.draw_sprite(canvas, 7 + anim_frame, 6, pos),
        }
    }

    pub fn draw_munch_death(&self, canvas: &mut graphics::Canvas, pos: glam::Vec2, frame: usize) {
        let anim_frame = ((frame / (DEATH_ANIM_FPS)) % 10) as u32;
        let sprite_sheet_x = anim_frame % 4;
        let sprite_sheet_y = anim_frame / 4;
        self.draw_sprite(canvas, 12 + sprite_sheet_x, 4 + sprite_sheet_y, pos)
    }

    pub fn draw_ghost(
        &self,
        canvas: &mut graphics::Canvas,
        ghost: &ghost::Ghost,
        pos: glam::Vec2,
        frame: usize,
    ) {
        match ghost.mode {
            ghost::Mode::Chase => {
                self.draw_ghost_chase(canvas, ghost, pos, frame);
            }
            ghost::Mode::Scatter => {
                self.draw_ghost_scatter(canvas, ghost, pos, frame);
            }
            ghost::Mode::Eaten => {
                self.draw_ghost_eaten(canvas, ghost, pos);
            }
        }
    }

    fn draw_ghost_chase(
        &self,
        canvas: &mut graphics::Canvas,
        ghost: &ghost::Ghost,
        pos: glam::Vec2,
        frame: usize,
    ) {
        let anim_frame = ((frame / ANIM_FPS) % 3) as u32;
        let y = match ghost.personality {
            ghost::Personality::Blinky => 0,
            ghost::Personality::Inky => 1,
            ghost::Personality::Pinky => 2,
            ghost::Personality::Clyde => 3,
        };
        match ghost.actor.move_direction {
            actor::Direction::Still | actor::Direction::Right => {
                self.draw_sprite(canvas, anim_frame, y, pos)
            }
            actor::Direction::Down => self.draw_sprite(canvas, 3 + anim_frame, y, pos),
            actor::Direction::Left => self.draw_sprite(canvas, 6 + anim_frame, y, pos),
            actor::Direction::Up => self.draw_sprite(canvas, 9 + anim_frame, y, pos),
        }
    }

    fn draw_ghost_scatter(
        &self,
        canvas: &mut graphics::Canvas,
        ghost: &ghost::Ghost,
        pos: glam::Vec2,
        frame: usize,
    ) {
        let anim_frame = ((frame / ANIM_FPS) % 3) as u32;
        let flash_frame = ((frame / (ANIM_FPS * 2)) % 2) as u32;
        match ghost.actor.move_direction {
            actor::Direction::Still | actor::Direction::Right => {
                self.draw_sprite(canvas, anim_frame, 4 + flash_frame, pos)
            }
            actor::Direction::Left => {
                self.draw_sprite(canvas, 3 + anim_frame, 4 + flash_frame, pos)
            }
            actor::Direction::Up => self.draw_sprite(canvas, 6 + anim_frame, 4 + flash_frame, pos),
            actor::Direction::Down => {
                self.draw_sprite(canvas, 9 + anim_frame, 4 + flash_frame, pos)
            }
        }
    }

    fn draw_ghost_eaten(
        &self,
        canvas: &mut graphics::Canvas,
        ghost: &ghost::Ghost,
        pos: glam::Vec2,
    ) {
        let y = match ghost.personality {
            ghost::Personality::Blinky => 0,
            ghost::Personality::Inky => 1,
            ghost::Personality::Pinky => 2,
            ghost::Personality::Clyde => 3,
        };
        match ghost.actor.move_direction {
            actor::Direction::Still | actor::Direction::Right => {
                self.draw_sprite(canvas, 12, y, pos)
            }
            actor::Direction::Down => self.draw_sprite(canvas, 13, y, pos),
            actor::Direction::Left => self.draw_sprite(canvas, 14, y, pos),
            actor::Direction::Up => self.draw_sprite(canvas, 15, y, pos),
        }
    }
}
