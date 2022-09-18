pub struct Stack<T> {
    vec: Vec<T>,
    size: u32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            vec: vec![],
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.vec.push(item);
        self.size += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.vec.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.vec.last()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
