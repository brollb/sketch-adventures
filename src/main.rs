extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx;
extern crate gfx_graphics;

extern crate vecmath;
extern crate image;
extern crate ggez;

use std::sync::mpsc;
use std::path::Path;
use std::fs::File;
use std::process::Command;
use std::{thread, time};
use std::borrow::BorrowMut;

use piston_window::*;
use gfx_graphics::GlyphCache;
use vecmath::*;
use image::Rgba;


use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;

mod resources;
mod player;
use player::Player;
mod enemy;
use enemy::Enemy;

enum GameState {
    Intro,
    Playing
}

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
    pub canvas: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    transmitter: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,

    // Intro
    state: GameState,
    settings: resources::Settings,
    pub message: Option<String>,
    message_position: (f64, f64),
    start_time: time::Instant
}

impl Game {
    pub fn new(width: f64, height: f64, settings: resources::Settings) -> Game {
        let player = Player::new(100.0, height - 80.0);
        let enemy = Enemy::new(200.0, 100.0);
        let empty_canvas = image::ImageBuffer::new(width as u32, height as u32);
        let (tx, rx) = mpsc::channel();
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
            canvas: empty_canvas,
            transmitter: tx,
            receiver: rx,
            message: Some("Incoming Transmission...".to_string()),
            settings: settings,
            state: GameState::Intro,
            start_time: time::Instant::now(),
            message_position: (10.0, 100.0)
        }
    }

    pub fn on_update(&mut self, upd: UpdateArgs) {
        // Detect collisions, etc
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

        match self.state {
            GameState::Intro => {
                let messages = [
                    ("Hello.", (400.0, 300.0)),
                    ("So, I've been stuck on this problem.", (300.0, 300.0)),
                    ("I was thinking about using *lightning*...", (300.0, 300.0)),
                    ("Or maybe something like a *clock*...", (300.0, 300.0)),
                    ("Think you could help me out?", (300.0, 300.0))
                ];

                let duration = time::Instant::now().duration_since(self.start_time);
                let index = (duration.as_secs()/2) as usize;
                if let Some(data) = messages.get(index) {
                    let &(message, position) = data;
                    self.message = Some(message.to_string());
                    self.message_position = position;
                } else {
                    self.message = None;
                    self.state = GameState::Playing;
                }
            },
            GameState::Playing => {
                if self.right_d {
                    self.player.mov(speed * dt, 0.0);
                }

                if self.left_d {
                    self.player.mov(-speed * dt, 0.0);
                }
            }
        }

        match self.receiver.try_recv() {
            Ok(msg) => self.create_drawing(&msg),
            _ => {
            }
        }
        
    }

    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let tank_sprite = assets.join("stick-person.png");
        let tank_sprite = Texture::from_path(
            &mut *w.factory.borrow_mut(),
            &tank_sprite,
            Flip::None,
            &TextureSettings::new())
            .unwrap();

        self.player.set_sprite(tank_sprite);

        let enemy_sprite = assets.join("E-100_preview.png");
        let enemy_sprite = Texture::from_path(
            &mut *w.factory.borrow_mut(),
            &enemy_sprite,
            Flip::None,
            &TextureSettings::new())
            .unwrap();

        self.enemy.set_sprite(enemy_sprite);
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
        self.last_pos = None;
    }

    fn create_drawing(&mut self, class: &str) {
        match class {
            _ => println!("creating a {}", class)
        }
    }

    fn on_drawing_complete(&mut self) {
        // Save the image to a file for now. In the future, we need to hand it off
        // for classification
        let buffer = self.canvas.clone();
        let tx = self.transmitter.clone();
        thread::spawn(move || {
            let filename = "drawing.png";
            let ref mut fout = File::create(filename).unwrap();
            image::ImageRgba8(buffer).save(fout, image::PNG).unwrap();
            println!("saved drawing to drawing.png");

            let output = Command::new("python")
                .arg("./src/doodle-classifier.py")
                .arg(filename)
                .output().unwrap_or_else(|e| {
                    panic!("Classification failed: {}", e)
                });

            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                let mut lines = s.rsplit("\n");
                lines.next();
                if let Some(line) = lines.next() {
                    if let Some(class) = line.split(",").next() {
                        tx.send(class.to_string()).unwrap();
                    }
                    /*
                    let conf = conf_string.parse::<f64>();
                    if conf > 0.5 {
                        self.create_drawing(class);
                    } else {
                        println!("thought it was {} but not sure ({})", class, conf);
                    }
                    */
                }

            } else {
                let s = String::from_utf8_lossy(&output.stderr);

                println!("failed. stderr was:\n{}", s);
            }
        });

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
                    let mut new_x = (last_x + (diff_x * delta)) as u32;
                    let mut new_y = (last_y + (diff_y * delta)) as u32;
                    let pen_size = 3;
                    new_x -= pen_size;
                    new_y -= pen_size;
                    if new_x < width && new_y < height {
                        for dx in 0..(2*pen_size + 1) {
                            for dy in 0..(2*pen_size + 1) {
                                self.canvas.put_pixel(new_x+dx, new_y+dy, Rgba([0, 0, 0, 255]));
                            }
                        }
                    };
                };
            };

            self.last_pos = Some(pos);
        }
    }

    pub fn on_draw(&mut self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer>) {
        // Redraw the screen (render each thing)
        clear([1.0; 4], g);
        match self.state {
            GameState::Playing =>  {
                self.player.render(c, g);
                self.enemy.render(c, g);
            }
            _ => {}
        }

        let text = graphics::Text::new(self.settings.font_size);
        if let Some(ref msg) = self.message {
            let (x, y) = self.message_position;
            let transform = c.transform.trans(x, y);
            text.draw(&msg.to_string(), &mut self.settings.font,
                      &c.draw_state, transform, g).unwrap();
        }

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

    // Load the necessary fonts...
    let font_path = Path::new("assets/Courier Prime.ttf");
    let factory = window.factory.clone();
    let font = GlyphCache::new(font_path, factory, TextureSettings::new()).unwrap();
    let settings = resources::Settings::new(font);

    let mut game = Game::new(width as f64, height as f64, settings);
    let mut texture = Texture::from_image(
            &mut window.factory,
            &image::ImageBuffer::new(width, height),
            &TextureSettings::new()
        ).unwrap();


    // Get the game font
    game.on_load(&mut window);

    //game.intro(&mut window);

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
