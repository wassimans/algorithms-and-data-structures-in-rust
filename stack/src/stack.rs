#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    // Put a new item in the tail of the inner Vec
    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    // Remove the last element of the inner Vec, return that element
    fn pop(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            _ => {
                self.size -= 1;
                self.data.pop()
            }
        }
    }

    // Return reference to the top value of the stack
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    // Return a mutable reference to the top value of the stack
    fn peek_mut(&mut self) -> Option<&mutT> {
        self.data.last_mut()
    }
}
