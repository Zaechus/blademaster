use tui::style::Color;

use crate::{CellAccess, CellKind};

#[derive(Clone, Debug, PartialEq)]
pub struct GameCell {
    x: i16,
    y: i16,
    kind: CellKind,
    name: String,
    color: Color,
    access: CellAccess,
}

impl GameCell {
    pub fn new(
        x: i16,
        y: i16,
        kind: CellKind,
        name: &str,
        color: Color,
        access: CellAccess,
    ) -> Self {
        Self {
            x,
            y,
            kind,
            name: name.to_string(),
            color,
            access,
        }
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

    pub fn inside(&self, x1: u16, y1: u16, x2: u16, y2: u16) -> bool {
        self.x >= x1 as i16 && self.y >= y1 as i16 && self.x <= x2 as i16 && self.y <= y2 as i16
    }

    pub fn x(&self) -> u16 {
        if self.x > 0 {
            self.x as u16
        } else {
            1
        }
    }
    pub fn y(&self) -> u16 {
        if self.y > 0 {
            self.y as u16
        } else {
            1
        }
    }
    pub fn kind(&self) -> CellKind {
        self.kind
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn access(&self) -> CellAccess {
        self.access
    }
}
