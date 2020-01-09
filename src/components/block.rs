use std::convert::TryInto;

use specs::prelude::*;

#[derive(Debug)]
pub struct Block {
    symbol: char,
    x: i32,
    y: i32,
}

impl Block {
    pub fn new(symbol: char, x: i32, y: i32) -> Self {
        Self { symbol, x, y }
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }
    pub fn move_down(&mut self) {
        self.y += 1;
    }
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
    pub fn x(&self) -> u16 {
        self.x.try_into().unwrap_or(1)
    }
    pub fn y(&self) -> u16 {
        self.y.try_into().unwrap_or(1)
    }
}

impl Component for Block {
    type Storage = VecStorage<Self>;
}
