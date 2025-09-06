use ggez::glam;
use ggez::graphics;
use ggez::Context;

use log::error;

use crate::{actor, config, ghost};

pub struct SpriteSheet {
    pub image: graphics::Image,
    sprite_u_width: f32,
    sprite_v_height: f32,
}

const SPRITESHEET_PATH: &str = "/spritesheet.png";
const SPRITE_WIDTH: u32 = 11;
const SPRITE_HEIGHT: u32 = 11;
const ANIM_FPS: usize = 60 / 5; // 5 frames per second

impl SpriteSheet {
    pub fn new(ctx: &mut Context) -> Self {
        let image = match graphics::Image::from_path(ctx, SPRITESHEET_PATH) {
            Ok(img) => img,
            Err(e) => {
                error!("Failed to load spritesheet image: {}", e);
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

    pub fn draw_munch(&self, canvas: &mut graphics::Canvas, pos: glam::Vec2) {
        self.draw_sprite(canvas, 2, 6, pos);
    }

    pub fn draw_ghost(
        &self,
        canvas: &mut graphics::Canvas,
        direction: actor::Direction,
        pos: glam::Vec2,
        frame: usize,
        personality: ghost::Personality,
    ) {
        let anim_frame = ((frame / ANIM_FPS) % 3) as u32;
        let y = match personality {
            ghost::Personality::Blinky => 0,
            ghost::Personality::Inky => 1,
            ghost::Personality::Pinky => 2,
            ghost::Personality::Clyde => 3,
        };
        match direction {
            actor::Direction::Still | actor::Direction::Right => {
                self.draw_sprite(canvas, anim_frame, y, pos)
            }
            actor::Direction::Down => self.draw_sprite(canvas, 3 + anim_frame, y, pos),
            actor::Direction::Left => self.draw_sprite(canvas, 6 + anim_frame, y, pos),
            actor::Direction::Up => self.draw_sprite(canvas, 9 + anim_frame, y, pos),
        }
    }
}
