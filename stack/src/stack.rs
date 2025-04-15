#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    // Put a new item in the tail of the inner Vec
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    // Remove the last element of the inner Vec, return that element
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    // Return reference to the top value of the stack
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    // Return a mutable reference to the top value of the stack
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    // Iteration over references of the Stack LIFO way
    pub fn iter(&self) -> std::iter::Rev<std::slice::Iter<T>> {
        self.data.iter().rev()
    }

    // Ietration over mutable references of the Stack LIFO way
    pub fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<T>> {
        self.data.iter_mut().rev()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Iteration implementations with LIFO style
// Used in for loops
//
// Does consume the Stack elements: stack.into_iter()
// Useful for example when we are done with the Stack
// and want to move the elemenst out.
// for item in stack ...
impl<T> IntoIterator for Stack<T> {
    type Item = T;

    type IntoIter = std::iter::Rev<std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().rev()
    }
}

// for item in &stack ...
impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;

    type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().rev()
    }
}

// for item in &mut stack ...
impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;

    type IntoIter = std::iter::Rev<std::slice::IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut s = Stack::new();
        s.push(10);
        assert_eq!(s.pop(), Some(10));
        assert!(s.is_empty());
    }
}
