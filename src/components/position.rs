use specs::prelude::*;

#[derive(Debug)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
