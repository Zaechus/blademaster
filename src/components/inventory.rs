use super::GameCell;

#[derive(Clone, Debug)]
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

    pub fn list(&self) -> Vec<String> {
        let mut list = Vec::with_capacity(self.cap);
        for gc in self.contents.iter() {
            list.push(gc.name());
        }
        list
    }

    pub fn take(&mut self, item: GameCell) {
        self.contents.push(item);
    }
}
