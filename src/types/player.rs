use std::borrow::Cow;

use tui::{
    style::{Color, Style},
    widgets::Text,
};

use crate::{CellAccess, GameCell};

pub struct Player {
    x: i32,
    y: i32,
    lvl: u32,
    hp: (i32, u32),
    xp: (i32, u32),
    sight: (i32, i32, i32, i32),
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            lvl: 1,
            hp: (10, 10),
            xp: (10, 10),
            sight: (4, 4, 4, 4),
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

    pub fn default_sight(&mut self) {
        self.sight = (4, 4, 4, 4);
    }
    pub fn reduce_sight(&mut self, gamecell: &GameCell, offset_x: i32, offset_y: i32) {
        if gamecell.access() == CellAccess::Impassable {
            if gamecell.inside(
                self.x() - self.sight().0,
                self.y(),
                self.x(),
                self.y(),
                offset_x,
                offset_y,
            ) {
                self.reduce_sight_0(self.x() - (gamecell.x() + offset_x));
            } else if gamecell.inside(
                self.x(),
                self.y() - self.sight().1,
                self.x(),
                self.y(),
                offset_x,
                offset_y,
            ) {
                self.reduce_sight_1(self.y() - (gamecell.y() + offset_y));
            } else if gamecell.inside(
                self.x(),
                self.y(),
                self.x() + self.sight().2,
                self.y(),
                offset_x,
                offset_y,
            ) {
                self.reduce_sight_2((gamecell.x() + offset_x) - self.x());
            } else if gamecell.inside(
                self.x(),
                self.y(),
                self.x(),
                self.y() + self.sight().3,
                offset_x,
                offset_y,
            ) {
                self.reduce_sight_3((gamecell.y() + offset_y) - self.y());
            }
        }
    }
    fn reduce_sight_0(&mut self, n: i32) {
        if n < self.sight.0 {
            self.sight.0 = n;
        }
    }
    fn reduce_sight_1(&mut self, n: i32) {
        if n < self.sight.1 {
            self.sight.1 = n;
        }
    }
    fn reduce_sight_2(&mut self, n: i32) {
        if n < self.sight.2 {
            self.sight.2 = n;
        }
    }
    fn reduce_sight_3(&mut self, n: i32) {
        if n < self.sight.3 {
            self.sight.3 = n;
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn sight(&self) -> (i32, i32, i32, i32) {
        self.sight
    }
}
