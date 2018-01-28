extern crate opengl_graphics;
extern crate piston_window;

use gfx_graphics::GlyphCache;
use gfx_device_gl::{Resources, Factory};

pub struct Settings {
    pub lightning_sprite: piston_window::Texture<Resources>,
    pub clock_sprite: piston_window::Texture<Resources>,
    pub goal_sprite: piston_window::Texture<Resources>,
    pub unknown_sprite: piston_window::Texture<Resources>,
    // TODO add other sprites
    pub font: GlyphCache<'static, Factory, Resources>,
    pub font_size: u32
}

impl Settings {
    pub fn new(
        font: GlyphCache<'static, Factory, Resources>,
        lightning_sprite: piston_window::Texture<Resources>,
        clock_sprite: piston_window::Texture<Resources>,
        unknown_sprite: piston_window::Texture<Resources>,
        goal_sprite: piston_window::Texture<Resources>
    ) -> Settings {
        Settings{
            lightning_sprite,
            clock_sprite,
            unknown_sprite,
            goal_sprite,
            font,
            font_size: 24
        }
    }
}
