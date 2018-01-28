extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, rectangle, image, Texture, Transformed};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use std::time;

use resources;

pub struct Lightning {
    pub alive: bool,
    sprite: Texture<Resources>,
    start_time: time::Instant,
    x: f64,
    y: f64,
}

impl Lightning {
    pub fn new(x: f64, y: f64, settings: &resources::Settings) -> Lightning {
        Lightning{
            start_time: time::Instant::now(),
            alive: true,
            sprite: settings.lightning_sprite.clone(),
            x,
            y
        }
    }

    pub fn update(&mut self, dt: f64) {
        let dx = 500.0 * dt;
        let dy = 500.0 * dt;
        self.x += dx;
        self.y += dy;
        self.alive = self.start_time.elapsed().as_secs() < 5;
    }

    pub fn render(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Draw player on the screen
        image(&self.sprite, c.transform.trans(self.x, self.y), g);
    }
}
