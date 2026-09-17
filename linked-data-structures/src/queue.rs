use crate::linked_list::LinkedList;

/// A queue backed by a linked list
pub struct Queue<T> {
    list: LinkedList<T>,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Queue { list: LinkedList::new() }
    }

    /// Add `data` to the end of the queue.
    pub fn enqueue(&mut self, data: T) {
        self.list.push_back(data);
    }

    /// Returns the value at the front of the queue or `None` if none exists.
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Returns the value at the front of the queue or `None` if none exists.
    pub fn peek(&self) -> Option<&T> {
        self.list.peek_front()
    }

    /// Returns `true` if the queue is empty and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_places_at_end() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn dequeue_is_first_in_first_out() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek_does_not_remove() {
        let mut queue = Queue::new();
        assert_eq!(queue.peek(), None);
        queue.enqueue(1);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn is_empty_works() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.dequeue();
        assert!(queue.is_empty());
    }
}
