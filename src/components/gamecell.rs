use bracket_lib::prelude::*;

use crate::types::{CellAccess, CellKind};

#[derive(Clone, Debug)]
pub struct GameCell {
    point: Point,
    kind: CellKind,
    name: String,
    color: RGB,
    access: CellAccess,
}

impl GameCell {
    pub fn new(point: Point, kind: CellKind, name: &str, color: RGB, access: CellAccess) -> Self {
        Self {
            point,
            kind,
            name: name.to_owned(),
            color,
            access,
        }
    }

    pub fn inside(&self, x1: i32, y1: i32, x2: i32, y2: i32, offset_x: i32, offset_y: i32) -> bool {
        let x = self.point.x + offset_x;
        let y = self.point.y + offset_y;
        x >= x1 && y >= y1 && x <= x2 && y <= y2
    }

    pub fn point(&self) -> Point {
        self.point
    }
    pub fn x(&self) -> i32 {
        self.point.x
    }
    pub fn y(&self) -> i32 {
        self.point.y
    }
    pub fn symbol(&self) -> char {
        self.kind.symbol()
    }
    /// Return the RGB color of the cell
    pub fn color(&self) -> RGB {
        self.color
    }
    /// Return a brightened version of the cell's color
    pub fn color_bright(&self) -> RGB {
        RGB::from_f32(self.color.r * 1.5, self.color.g * 1.5, self.color.b * 1.5)
    }
    /// Return a black background for the cell
    pub fn bg_color(&self) -> RGB {
        RGB::new()
    }
    pub fn kind(&self) -> CellKind {
        self.kind
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn access(&self) -> CellAccess {
        self.access
    }
}
