use std::collections::VecDeque;

#[derive(Clone, Default, Debug)]
pub struct GameEvents {
    events: VecDeque<String>,
}

impl GameEvents {
    pub fn new() -> Self {
        let mut events = VecDeque::with_capacity(5);
        for _ in 0..5 {
            events.push_back(String::from("\n"));
        }
        Self { events }
    }

    pub fn post_event(&mut self, content: String) {
        self.events.pop_front();
        self.events.push_back(String::from(content));
    }

    pub fn events(&self) -> &VecDeque<String> {
        &self.events
    }
}
