//! # Queue
//!
//! A simple, generic **First-In-First-Out (FIFO)** queue implemented using Rust’s `VecDeque<T>`.
//!
//! This queue data structure supports standard queue operations and safe, idiomatic iteration.
//!
//! ## Features
//!
//! - Enqueue items at the back (`enqueue`)
//! - Dequeue items from the front (`dequeue`)
//! - Peek at the front item (with mutable or shared reference)
//! - Check if the queue is empty (`is_empty`) or get its length (`len`)
//! - Clear all items from the queue (`clear`)
//! - Iterate immutably or mutably with `.iter()` / `.iter_mut()`
//! - Use in `for` loops with `IntoIterator` support (by value, shared, or mutable reference)
//!
//! ## Example
//!
//! ```rust
//! use my_crate::Queue;
//!
//! let mut queue = Queue::new();
//! queue.enqueue(10);
//! queue.enqueue(20);
//!
//! assert_eq!(queue.peek(), Some(&10));
//! assert_eq!(queue.dequeue(), Some(10));
//! assert_eq!(queue.dequeue(), Some(20));
//! assert!(queue.is_empty());
//! ```
//!
//! ## Iteration
//!
//! You can use any of the following idioms:
//!
//! ```rust
//! for value in &queue {}       // Immutable borrow
//! for value in &mut queue {}   // Mutable borrow
//! for value in queue {}        // By value, consumes the queue
//! ```

use std::collections::{
    vec_deque::{Iter, IterMut},
    VecDeque,
};

#[derive(Debug)]
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
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

    pub fn enqueue(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;

    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;

    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;

    type IntoIter = std::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut q: Queue<u32> = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);
        q.enqueue(7);
        assert_eq!(q.dequeue(), Some(1));

        let peek_mut = q.peek_mut();
        assert_eq!(*peek_mut.unwrap(), 2);

        let sum = q.iter().sum::<u32>();
        assert_eq!(sum, 27);

        q.clear();
        assert!(q.is_empty());
    }
}
