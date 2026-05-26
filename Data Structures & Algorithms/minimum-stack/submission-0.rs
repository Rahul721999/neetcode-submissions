struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let min = if self.stack.is_empty() {
            val
        } else {
            self.get_min().min(val)
        };
        self.stack.push((val, min));
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
        }
    }

    pub fn top(&mut self) -> i32 {
        if !self.stack.is_empty() {
            self.stack.last().unwrap().0
        } else {
            panic!("array is empty")
        }
    }

    pub fn get_min(&self) -> i32 {
        if !self.stack.is_empty() {
            self.stack.last().unwrap().1
        } else {
            panic!("array is empty")
        }
    }
}

