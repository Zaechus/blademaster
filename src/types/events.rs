use std::collections::VecDeque;

use bracket_lib::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct GameEvents {
    events: VecDeque<(String, RGB)>,
}

impl GameEvents {
    pub fn new() -> Self {
        let mut events = VecDeque::with_capacity(5);
        for _ in 0..5 {
            events.push_back((String::from("\n"), RGB::new()));
        }
        Self { events }
    }

    pub fn post_event(&mut self, content: String, color: RGB) {
        self.events.pop_front();
        self.events.push_back((content, color));
    }

    pub fn print(&self, ctx: &mut BTerm, window_size: (u32, u32)) {
        for (y, s) in self.events.iter().enumerate() {
            ctx.print_color(
                1,
                window_size.1 as i32 - 6 + y as i32,
                s.1,
                RGB::new(),
                &s.0,
            );
        }
    }
}
