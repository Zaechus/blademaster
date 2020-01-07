use specs::prelude::*;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
