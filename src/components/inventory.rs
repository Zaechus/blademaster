use bracket_lib::prelude::*;

use super::GameCell;

#[derive(Clone, Debug)]
pub struct Inventory {
    shown: bool,
    cap: usize,
    contents: Vec<GameCell>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            shown: false,
            cap: 10,
            contents: Vec::with_capacity(10),
        }
    }

    pub fn print(&self, ctx: &mut BTerm, window_size: (u32, u32)) {
        if self.shown {
            ctx.draw_box(
                window_size.0 as i32 - window_size.0 as i32 / 4 - 2,
                1,
                window_size.0 as i32 / 4,
                window_size.1 as i32 - 3,
                RGB::from_u8(0, 170, 0),
                RGB::from_u8(100, 100, 100),
            );
            for (y, gc) in self.contents.iter().enumerate() {
                ctx.print(
                    window_size.0 as i32 - window_size.0 as i32 / 4 - 1,
                    2 + y as i32,
                    gc.name(),
                );
            }
        }
    }

    pub fn toggle(&mut self) {
        self.shown = !self.shown;
    }

    pub fn take(&mut self, item: GameCell) {
        self.contents.push(item);
    }
}
