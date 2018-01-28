extern crate opengl_graphics;

use gfx_graphics::GlyphCache;
use std::path::Path;

pub struct Resources {
    pub font: GlyphCache,
    pub font_size: u32
}

impl Resources {
    pub fn new() -> Resources {
        let font_path = Path::new("assets/Verdana.ttf");
        let ref mut font = GlyphCache::new(font_path).unwrap();
        Resources{
            font,
            font_size: 64
        }
    }
}
