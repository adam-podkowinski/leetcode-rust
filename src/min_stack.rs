#![allow(dead_code)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || self.min_stack.last() >= Some(&val) {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        let popped = self.stack.pop().unwrap();
        if self.min_stack.last() == Some(&popped) {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().cloned().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().cloned().unwrap()
    }
}
