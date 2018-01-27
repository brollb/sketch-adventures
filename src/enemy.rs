extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, rectangle, image, Texture, Transformed};
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

// Can't declare a module here... why?
//mod point;
//use point::Point;
pub struct Point {
    x: f64,
    y: f64
}

pub struct Enemy {
    sprite: Option<Texture<Resources>>,
    origin: Point,
    dir: f64,
    x: f64,
    y: f64
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy{
            sprite: None,
            origin: Point{x, y},
            dir: 1.0,
            x, y
        }
    }

    pub fn update(&mut self, dt: f64) {  // Should I also pass some global info here?
        let max_distance = 100.0;
        if self.x > self.origin.x + max_distance {
            self.dir = -1.0;
        }
        if self.x < self.origin.x - max_distance {
            self.dir = 1.0;
        }

        self.x = self.x + (50.0 * self.dir * dt);
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
