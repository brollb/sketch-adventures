extern crate opengl_graphics;
extern crate piston_window;

use std::collections::HashMap;
use gfx_graphics::GlyphCache;
use gfx_device_gl::{Resources, Factory};

pub struct Settings {
    default_sprite: piston_window::Texture<Resources>,
    pub sprites: HashMap<String, piston_window::Texture<Resources>>,
    // TODO add other sprites
    pub font: GlyphCache<'static, Factory, Resources>,
    pub font_size: u32
}

impl Settings {
    pub fn new(
        font: GlyphCache<'static, Factory, Resources>,
        sprites: HashMap<String, piston_window::Texture<Resources>>,
        default_sprite: piston_window::Texture<Resources>
    ) -> Settings {
        Settings{
            default_sprite,
            sprites,
            font,
            font_size: 24
        }
    }

    pub fn get_sprite_for(&self, name: &str) -> piston_window::Texture<Resources> {
        println!("Getting sprite for '{}'", name);
        match self.sprites.get(name) {
            Some(sprite) => sprite.clone(),
            None => self.default_sprite.clone()
        }
    }
}
