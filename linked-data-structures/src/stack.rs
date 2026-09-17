use crate::linked_list::LinkedList;

/// A stack backed by a linked list
pub struct Stack<T> {
    list: LinkedList<T>,
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Stack { list: LinkedList::new() }
    }

    /// Add `data` to the top of the stack.
    pub fn push(&mut self, data: T) {
        self.list.push_front(data);
    }

    /// Returns the value at the top of the stack or `None` if none exists.
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Returns the value on the top of the stack or `None` if none exists.
    pub fn peek(&self) -> Option<&T> {
        self.list.peek_front()
    }

    /// Reverses the stack
    pub fn reverse(&mut self) {
        self.list.reverse();
    }

    /// Returns `true` if the stack is empty and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_places_on_top() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn pop_is_last_in_first_out() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_does_not_remove() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn is_empty_works() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn reverse_puts_bottom_on_top() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.reverse();
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), None);
    }
}
