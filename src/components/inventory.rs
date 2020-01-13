#[derive(Debug, Default)]
pub struct Inventory {
    cap: usize,
    contents: Vec<char>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            cap: 10,
            contents: Vec::with_capacity(10),
        }
    }
}
