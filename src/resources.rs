extern crate opengl_graphics;

use gfx_graphics::GlyphCache;
use gfx_device_gl::{Resources, Factory};

pub struct Settings {
    pub font: GlyphCache<'static, Factory, Resources>,
    pub font_size: u32
}

impl Settings {
    pub fn new(font: GlyphCache<'static, Factory, Resources>) -> Settings {
        Settings{
            font,
            font_size: 64
        }
    }
}
