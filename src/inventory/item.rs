#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    name: String,
    value: u32,
}

impl Item {
    pub fn new(name: String, value: u32) -> Self {
        Item { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
