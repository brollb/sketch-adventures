extern crate rand;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, rectangle, image, Texture, Transformed};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

pub struct Enemy {
    sprite: Option<Texture<Resources>>,
    x: f64,
    min_x: f64,
    max_x: f64,
    max_y: f64,
    y: f64
}

impl Enemy {
    pub fn new(min_x: f64, max_x: f64, max_y: f64) -> Enemy {
        Enemy{
            sprite: None,
            min_x: min_x,
            max_x: max_x,
            max_y: max_y,
            x: 0.0,
            y: max_y + 1.0
        }
    }

    pub fn update(&mut self, dt: f64) {  // Should I also pass some global info here?
        // Animate falling
        let speed = 700.0;
        if self.y > self.max_y {
            self.reset_position();
        } else {
            self.y += speed * dt;
        }
    }

    fn reset_position(&mut self) {
        self.y = -500.0;
        let diff = self.max_x - self.min_x;
        let rand_in_range = rand::random::<f64>() * diff;
        self.x = rand_in_range + self.min_x;
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
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
