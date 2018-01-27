pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player{x, y}
    }
}
