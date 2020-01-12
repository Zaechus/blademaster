use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Inventory {
    cap: usize,
    contents: Vec<Entity>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            cap: 10,
            contents: Vec::with_capacity(10),
        }
    }
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}
