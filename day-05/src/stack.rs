#[derive(Debug, Clone)]
pub struct Stack {
    pub items: Vec<char>,
}

impl Stack {
    pub fn push(&mut self, item: char) {
        self.items.push(item);
    }

    pub fn push_n(&mut self, items: Vec<char>) {
        self.items.extend(items);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<char> {
        self.items.split_off(self.items.len() - n)
    }

    pub fn peek(&self) -> Option<char> {
        self.items.last().copied()
    }
}
