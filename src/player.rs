extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, rectangle};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

pub struct Player {
    pub x: f64,
    pub y: f64
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player{x, y}
    }

    pub fn render(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Draw player on the screen
        rectangle([1.0, 0.0, 0.0, 1.0], // red
                  [self.x, self.y, 100.0, 100.0],
                  c.transform, g);
    }
}
