use std::borrow::Cow;

use tui::{
    style::{Color, Style},
    widgets::Text,
};

use crate::GameCell;

#[derive(Clone, Debug, Default)]
pub struct Inventory {
    cap: usize,
    contents: Vec<GameCell>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            cap: 10,
            contents: Vec::with_capacity(10),
        }
    }

    pub fn list<'a>(&self) -> Vec<Text<'a>> {
        let mut list = Vec::with_capacity(self.cap);
        for gc in self.contents.iter() {
            list.push(Text::Styled(
                Cow::from(gc.name()),
                Style::default().fg(Color::Blue),
            ));
        }
        list
    }

    pub fn take(&mut self, item: GameCell) {
        self.contents.push(item);
    }
}
