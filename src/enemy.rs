extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::{Context, rectangle};
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
    origin: Point,
    dir: f64,
    x: f64,
    y: f64
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy{
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

    pub fn render(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        rectangle([0.0, 1.0, 0.0, 1.0], // red
                  [self.x, self.y, 50.0, 50.0],
                  c.transform, g);
    }
}
