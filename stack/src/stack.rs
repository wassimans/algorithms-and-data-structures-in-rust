//! # Stack
//!
//! A simple and efficient Last-In-First-Out (LIFO) stack implementation built on top of `Vec<T>`.
//!
//! This `Stack<T>` provides a classic stack API with additional iterator support. Internally, it
//! stores elements using a `Vec<T>`, treating the **end of the vector as the top of the stack**.
//!
//! All operations are O(1) amortized time, leveraging Rust's standard `Vec<T>`.
//!
//! ## Features
//! - `new()` / `default()` — Create a new empty stack
//! - `push()` — Add an item to the top
//! - `pop()` — Remove and return the top item
//! - `peek()` / `peek_mut()` — Look at the top item (immutable or mutable)
//! - `clear()` — Remove all items
//! - `len()` / `is_empty()` — Inspect size
//! - `iter()` / `iter_mut()` — Iterate in LIFO order
//! - Supports `for` loops by implementing `IntoIterator`
//!
//! ## Example
//!
//! ```rust
//! use stack::Stack;
//!
//! let mut s = Stack::new();
//! s.push(1);
//! s.push(2);
//! assert_eq!(s.peek(), Some(&2));
//!
//! for value in &s {
//!     println!("Stack value: {value}");
//! }
//!
//! while let Some(top) = s.pop() {
//!     println!("Popped: {top}");
//! }
//! ```
//!
//! ## LIFO Iteration
//! All iterators traverse the stack in **LIFO order** (last inserted item first):
//!
//! ```rust
//! use stack::Stack;
//!
//! let mut stack = Stack::new();
//! stack.push(1);
//! stack.push(2);
//!
//! for item in &stack {  // immutable
//!     println!("{item}");
//! }
//!
//! for item in &mut stack {  // mutable
//!     *item *= 2;
//! }
//!
//! for item in stack {  // by-value, consuming the stack
//!     println!("{item}");
//! }
//! ```

#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    pub fn iter(&self) -> std::iter::Rev<std::slice::Iter<T>> {
        self.data.iter().rev()
    }

    pub fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<T>> {
        self.data.iter_mut().rev()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;

    type IntoIter = std::iter::Rev<std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().rev()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;

    type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().rev()
    }
}

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
    fn test_basic_operations() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);
        s.push(6);
        s.push(7);
        assert_eq!(s.pop(), Some(7));

        let peek_mut = s.peek_mut();
        assert_eq!(*peek_mut.unwrap(), 6);

        let sum = s.iter().sum::<i32>();
        assert_eq!(sum, 21);

        s.clear();
        assert!(s.is_empty());
    }
}
