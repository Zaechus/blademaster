use std::collections::VecDeque;

use tui::{
    style::{Color, Style},
    widgets::Text,
};

#[derive(Clone, Default, Debug)]
pub struct GameEvents<'a> {
    events: VecDeque<Text<'a>>,
}

impl<'a> GameEvents<'a> {
    pub fn new() -> Self {
        let mut events = VecDeque::with_capacity(5);
        for _ in 0..5 {
            events.push_back(Text::styled("\n", Style::default()));
        }
        Self { events }
    }

    pub fn post_event(&mut self, content: String, color: Color) {
        self.events.pop_front();
        self.events
            .push_back(Text::styled(content, Style::default().fg(color)));
    }

    pub fn events(&self) -> &VecDeque<Text<'a>> {
        &self.events
    }
}
