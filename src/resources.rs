extern crate opengl_graphics;
extern crate piston_window;

use gfx_graphics::GlyphCache;
use gfx_device_gl::{Resources, Factory};

pub struct Settings {
    pub lightning_sprite: piston_window::Texture<Resources>,
    pub clock_sprite: piston_window::Texture<Resources>,
    pub font: GlyphCache<'static, Factory, Resources>,
    pub font_size: u32
}

impl Settings {
    pub fn new(
        font: GlyphCache<'static, Factory, Resources>,
        lightning_sprite: piston_window::Texture<Resources>,
        clock_sprite: piston_window::Texture<Resources>
    ) -> Settings {
        Settings{
            lightning_sprite,
            clock_sprite,
            font,
            font_size: 24
        }
    }
}
