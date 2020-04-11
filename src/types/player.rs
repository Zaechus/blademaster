use bracket_lib::prelude::*;

pub struct Player {
    point: Point,
    lvl: u32,
    hp: (i32, u32),
    xp: (i32, u32),
    sight: (i32, i32, i32, i32),
}

impl Player {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            lvl: 1,
            hp: (10, 10),
            xp: (0, 10),
            sight: (4, 4, 4, 4),
        }
    }

    pub fn print_info(&self, ctx: &mut BTerm, window_size: (u32, u32)) {
        ctx.print(
            window_size.0 as i32 - 15,
            window_size.1 as i32 - 4,
            format!("Level: {}", self.lvl),
        );
        ctx.print(
            window_size.0 as i32 - 15,
            window_size.1 as i32 - 3,
            format!("HP: {} / {}", self.hp.0, self.hp.1),
        );
        ctx.print(
            window_size.0 as i32 - 15,
            window_size.1 as i32 - 2,
            format!("XP: {} / {}", self.xp.0, self.xp.1),
        );
    }

    pub fn move_pos(&mut self, a: i32, b: i32) {
        self.point.x += a;
        self.point.y += b;
    }

    pub fn default_sight(&mut self) {
        self.sight = (4, 4, 4, 4);
    }

    pub fn x(&self) -> i32 {
        self.point.x
    }
    pub fn y(&self) -> i32 {
        self.point.y
    }
    pub fn sight(&self) -> (i32, i32, i32, i32) {
        self.sight
    }
}
