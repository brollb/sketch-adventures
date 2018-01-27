extern crate piston;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

use piston_window::*;
use piston::input::GenericEvent;

use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

mod player;
use player::Player;

struct Game {
    player: Player
}

impl Game {
    pub fn new() -> Game {
        let player = Player::new(100.0, 100.0);
        Game{player}
    }

    pub fn on_update(&self, upd: UpdateArgs) {
        // Detect collisions, etc
        // TODO
    }

    pub fn on_input(&mut self, input: piston::input::Button) {
        // Update the game state
        let speed = 5.0;
        if let Button::Keyboard(Key::Left) = input {
            self.player.x -= speed;
        }

        if let Button::Keyboard(Key::Right) = input {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            self.player.x += speed;
        }

        if let Button::Keyboard(Key::Up) = input {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            self.player.y -= speed;
        }

        if let Button::Keyboard(Key::Down) = input {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            self.player.y += speed;
        }
    }

    pub fn on_draw(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Redraw the screen (render each thing)
        clear([1.0; 4], g);
        self.player.render(c, g)
    }
}

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    while let Some(e) = window.next() {
        // Split this into update and render events as done
        //  at http://piston-tutorial.logdown.com/posts/306682

        if let Some(input) = e.press_args() {
            game.on_input(input);
        }

        window.draw_2d(&e, |c, g| {
            game.on_draw(c, g);
        });

        if let Some(args) = e.update_args() {
            game.on_update(args);
        }
    }
}
