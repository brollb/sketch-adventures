extern crate piston;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx;
extern crate gfx_graphics;

extern crate vecmath;
extern crate image;

use piston_window::*;
use std::borrow::BorrowMut;
use vecmath::*;
use image::Rgba;


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
    left_d: bool,

    // Drawing
    is_drawing: bool,
    last_pos: Option<[f64; 2]>,
    pub canvas: image::ImageBuffer<Rgba<u8>, Vec<u8>>
}

impl Game {
    pub fn new(width: f64, height: f64) -> Game {
        let player = Player::new(100.0, 100.0);
        let enemy = Enemy::new(200.0, 100.0);
        let empty_canvas = image::ImageBuffer::new(width as u32, height as u32);
        Game{
            player,
            enemy,
            width,
            height,
            up_d: false,
            down_d: false,
            right_d: false,
            left_d: false,
            is_drawing: false,
            last_pos: None,
            canvas: empty_canvas
        }
    }

    pub fn on_update(&mut self, upd: UpdateArgs) {
        // Detect collisions, etc
        // TODO
        let dt = upd.dt;
        let speed = 100.0;
        self.enemy.update(dt);

        // TODO: get the window encoder...
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

    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let tank_sprite = assets.join("E-100_preview.png");
        let tank_sprite = Texture::from_path(
            &mut *w.factory.borrow_mut(),
            &tank_sprite,
            Flip::None,
            &TextureSettings::new())
            .unwrap();

        self.player.set_sprite(tank_sprite);
    }

    pub fn on_release(&mut self, input: piston::input::Button) {
        // Player movement
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

        // Painting
        if input == Button::Mouse(MouseButton::Left) {
            println!("stopping drawing!");
            self.is_drawing = false;
            self.on_drawing_complete();
        }
    }

    pub fn on_press(&mut self, input: piston::input::Button) {
        // Player movement
        if let Button::Keyboard(Key::Left) = input {
            self.left_d = true;
        }

        if let Button::Keyboard(Key::Right) = input {
            self.right_d = true;
        }

        if let Button::Keyboard(Key::Up) = input {
            self.up_d = true;
        }

        if let Button::Keyboard(Key::Down) = input {
            self.down_d = true;
        }

        // Painting
        if input == Button::Mouse(MouseButton::Left) {
            println!("started drawing!");
            self.is_drawing = true;
            self.clear_drawing();
        }
    }

    fn clear_drawing(&mut self) {
        let width = self.width as u32;
        let height = self.height as u32;
        self.canvas = image::ImageBuffer::new(width, height);
    }

    fn on_drawing_complete(&mut self) {
        // Get the image and detect stuff
        // TODO
        // Create an entity of the given type if needed
        // TODO
        self.clear_drawing();
    }

    pub fn on_mouse_move(&mut self, pos: [f64; 2]) {
        let width = self.width as u32;
        let height = self.height as u32;

        if self.is_drawing {
            let (x, y) = (pos[0] as f32, pos[1] as f32);

            if let Some(p) = self.last_pos {
                let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                let distance = vec2_len(vec2_sub(p, pos)) as u32;

                for i in 0..distance {
                    let diff_x = x - last_x;
                    let diff_y = y - last_y;
                    let delta = i as f32 / distance as f32;
                    let new_x = (last_x + (diff_x * delta)) as u32;
                    let new_y = (last_y + (diff_y * delta)) as u32;
                    if new_x < width && new_y < height {
                        self.canvas.put_pixel(new_x, new_y, Rgba([0, 0, 0, 255]));
                    };
                };
            };

            self.last_pos = Some(pos);
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
    let mut texture = Texture::from_image(
            &mut window.factory,
            &image::ImageBuffer::new(width, height),
            &TextureSettings::new()
        ).unwrap();

    game.on_load(&mut window);
    while let Some(e) = window.next() {
        // Split this into update and render events as done
        //  at http://piston-tutorial.logdown.com/posts/306682

        if let Some(input) = e.release_args() {
            game.on_release(input);
        }

        if let Some(input) = e.press_args() {
            game.on_press(input);
        }

        texture = Texture::from_image(
                &mut window.factory,
                &game.canvas,
                &TextureSettings::new()
            ).unwrap();
        window.draw_2d(&e, |c, g| {
            // Detect drawing...
            game.on_draw(c, g);

            // Display any drawing
            image(&texture, c.transform, g);
        });

        if let Some(args) = e.update_args() {
            game.on_update(args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            game.on_mouse_move(pos);
        }
    }
}
