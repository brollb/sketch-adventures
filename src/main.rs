extern crate piston_window;

use piston_window::*;

mod player;
use player::Player;

struct Game {
    player: Player
}

impl Game {
    pub fn new() -> Game {
        let player = Player::new(100, 100);
        Game{player}
    }

    pub fn on_update(&self, upd: UpdateArgs) {
        println!("Updating Game!!!");
    }

    /*
    pub fn on_draw(&self, ren: Event, window: PistonWindow) {
        // Redraw the screen
        window.draw_2d(&ren, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
    */
}

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let game = Game::new();
    let mut x = 0.0;
    let mut y = 0.0;
    let speed = 5.0;

    while let Some(e) = window.next() {
        // Split this into update and render events as done
        //  at http://piston-tutorial.logdown.com/posts/306682

        // Update the game state
        if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            println!("Left!");
            x -= speed;
        }

        if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            println!("Right!");
            x += speed;
        }

        if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            println!("Up!");
            y += speed;
        }

        if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
                //let Some(Button::Keyboard(Key::A)) = e.press_args() {
            println!("Down!");
            y += speed;
        }

        // TODO: move this to the on_draw for the world...
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x, y, 100.0, 100.0],
                      c.transform, g);
        });

        /*
        match e {
            Some(Event::UpdateEvent(event)) => {
                //if !event.is_none() {
                    game.on_update(event.update_args());
                //}
            }
            //Some(Event::RenderEvent(ren)) => {
                //game.on_draw(ren, window);
            //}
        }
        */
    }
}
