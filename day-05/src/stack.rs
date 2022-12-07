#[derive(Debug, Clone)]
pub struct Stack {
    pub items: Vec<char>,
}

impl Stack {
    pub fn push(&mut self, item: char) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<char> {
        self.items.last().copied()
    }
}
