use std::borrow::Cow;

use tui::{
    style::{Color, Style},
    widgets::Text,
};

pub struct Player {
    x: f64,
    y: f64,
    lvl: u32,
    hp: (i32, u32),
    xp: (i32, u32),
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            lvl: 1,
            hp: (10, 10),
            xp: (10, 10),
        }
    }

    pub fn list<'a>(&self) -> Vec<Text<'a>> {
        let mut list = Vec::with_capacity(3);
        list.push(Text::Styled(
            Cow::from(format!("Level: {}", self.lvl)),
            Style::default().fg(Color::Blue),
        ));
        list.push(Text::Styled(
            Cow::from(format!("HP: {} / {}", self.hp.0, self.hp.1)),
            Style::default().fg(Color::Blue),
        ));
        list.push(Text::Styled(
            Cow::from(format!("XP: {} / {}", self.xp.0, self.xp.1)),
            Style::default().fg(Color::Blue),
        ));
        list
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}
