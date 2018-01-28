extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, image, Texture, Transformed};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

use resources;

pub struct Goal {
    sprite: Texture<Resources>,
    pub x: f64,
    pub y: f64
}

impl Goal {
    pub fn new(x: f64, y: f64, settings: &resources::Settings) -> Goal {
        Goal{
            sprite: settings.get_sprite_for("goal"),
            x,
            y
        }
    }

    pub fn render(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        image(&self.sprite, c.transform.trans(self.x, self.y), g);
    }
}
