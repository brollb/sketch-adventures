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
mod enemy;
use enemy::Enemy;

struct Game {
    width: f64,
    height: f64,
    player: Player,
    enemy: Enemy,

    // Relevant Key states
    up_d: bool,
    down_d: bool,
    right_d: bool,
    left_d: bool
}

impl Game {
    pub fn new(width: f64, height: f64) -> Game {
        let player = Player::new(100.0, 100.0);
        let enemy = Enemy::new(200.0, 100.0);
        Game{
            player,
            enemy,
            width,
            height,
            up_d: false,
            down_d: false,
            right_d: false,
            left_d: false
        }
    }

    pub fn on_update(&mut self, upd: UpdateArgs) {
        // Detect collisions, etc
        // TODO
        let dt = upd.dt;
        let speed = 100.0;
        self.enemy.update(dt);

        /*
        if self.up_d {
            self.player.mov(0.0, speed * dt);
        }

        if self.down_d {
            self.player.mov(0.0, -speed * dt);
        }
        */

        if self.right_d {
            self.player.mov(speed * dt, 0.0);
        }

        if self.left_d {
            self.player.mov(-speed * dt, 0.0);
        }
    }

    pub fn on_release(&mut self, input: piston::input::Button) {
        if let Button::Keyboard(Key::Left) = input {
            self.left_d = false;
        }

        if let Button::Keyboard(Key::Right) = input {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            self.right_d = false;
        }

        if let Button::Keyboard(Key::Up) = input {
            self.up_d = false;
        }

        if let Button::Keyboard(Key::Down) = input {
            self.down_d = false;
        }
    }

    pub fn on_press(&mut self, input: piston::input::Button) {
        // Update the game state
        let speed = 5.0;

        if let Button::Keyboard(Key::Left) = input {
            self.left_d = true;
        }

        if let Button::Keyboard(Key::Right) = input {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            self.right_d = true;
        }

        if let Button::Keyboard(Key::Up) = input {
            self.up_d = true;
        }

        if let Button::Keyboard(Key::Down) = input {
            self.down_d = true;
        }
    }

    pub fn on_draw(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Redraw the screen (render each thing)
        clear([1.0; 4], g);
        self.player.render(c, g);
        self.enemy.render(c, g);

        // Draw the ground
        rectangle([0.3, 0.3, 0.3, 1.0], // black
                  [0.0, self.height - 20.0, self.width*100.0, 100.0],
                  c.transform, g);
    }
}

fn main() {
    let (width, height) = (1280, 960);
    let mut window: PistonWindow = 
        WindowSettings::new("To Be Determined", [width, height])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width as f64, height as f64);

    while let Some(e) = window.next() {
        // Split this into update and render events as done
        //  at http://piston-tutorial.logdown.com/posts/306682

        if let Some(input) = e.release_args() {
            game.on_release(input);
        }

        if let Some(input) = e.press_args() {
            game.on_press(input);
        }

        window.draw_2d(&e, |c, g| {
            game.on_draw(c, g);
        });

        if let Some(args) = e.update_args() {
            game.on_update(args);
        }
    }
}
