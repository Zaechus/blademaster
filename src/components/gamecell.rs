use tui::style::Color;

use crate::{CellAccess, CellKind};

#[derive(Clone, Debug, PartialEq)]
pub struct GameCell {
    x: i32,
    y: i32,
    kind: CellKind,
    name: String,
    color: Color,
    access: CellAccess,
}

impl GameCell {
    pub fn new(
        x: i32,
        y: i32,
        kind: CellKind,
        name: &str,
        color: Color,
        access: CellAccess,
    ) -> Self {
        Self {
            x,
            y,
            kind,
            name: name.to_owned(),
            color,
            access,
        }
    }

    pub fn inside(&self, x1: i32, y1: i32, x2: i32, y2: i32, offset_x: i32, offset_y: i32) -> bool {
        let x = self.x + offset_x;
        let y = self.y + offset_y;
        x >= x1 && y >= y1 && x <= x2 && y <= y2
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
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
