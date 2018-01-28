extern crate piston_window;

use piston_window::{Context, rectangle, image, Texture};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use piston_window::Transformed;

pub struct Player {
    pub alive: bool,
    pub x: f64,
    pub y: f64,
    sprite: Option<Texture<Resources>>,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player{alive: true, x, y, sprite: None}
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }

    pub fn mov(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn render(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Draw player on the screen
        match self.sprite {
            None => {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [self.x, self.y, 100.0, 100.0],
                          c.transform, g);
            }
            Some(ref sprite) => {
                // Draw at x, y position
                image(sprite, c.transform.trans(self.x, self.y), g);
            }
        }
    }
}
