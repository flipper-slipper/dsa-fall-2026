use crate::min_heap::MinHeap;

/// A priority queue for dijkstras based on a min-heap.
pub struct PriorityQueue<T> {
    heap: MinHeap<T>,
}

impl<T: PartialEq> PriorityQueue<T> {
    /// Creates an empty priority queue.
    pub fn new() -> Self {
        PriorityQueue { heap: MinHeap::new() }
    }

    /// Returns `true` if the queue is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Add `elem` at level `priority`.
    pub fn add_with_priority(&mut self, elem: T, priority: f64) {
        self.heap.insert(elem, priority);
    }

    /// Get the next (highest priority) element and remove this element from
    /// the queue.
    ///
    /// Returns the next element in terms of priority. If empty, returns
    /// `None`.
    pub fn next(&mut self) -> Option<T> {
        self.heap.pop_min()
    }

    /// Adjust the priority of the given element.
    ///
    /// `elem` is the element whose priority should change, and `new_priority`
    /// is the priority to use for the element. The lower the priority, the
    /// earlier the element is in the order.
    pub fn adjust_priority(&mut self, elem: &T, new_priority: f64) {
        self.heap.adjust_priority(elem, new_priority);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let queue: PriorityQueue<&str> = PriorityQueue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn is_empty_is_false_after_adding() {
        let mut queue = PriorityQueue::new();
        queue.add_with_priority("a", 1.0);
        assert!(!queue.is_empty());
    }

    #[test]
    fn add_with_priority_adds_element() {
        let mut queue = PriorityQueue::new();
        queue.add_with_priority("a", 1.0);
        assert_eq!(queue.next(), Some("a"));
    }

    #[test]
    fn next_returns_lowest_priority_first() {
        let mut queue = PriorityQueue::new();
        queue.add_with_priority("b", 2.0);
        queue.add_with_priority("a", 1.0);
        assert_eq!(queue.next(), Some("a"));
        assert_eq!(queue.next(), Some("b"));
        assert_eq!(queue.next(), None);
    }

    #[test]
    fn adjust_priority_changes_order() {
        let mut queue = PriorityQueue::new();
        queue.add_with_priority("a", 1.0);
        queue.add_with_priority("b", 2.0);
        queue.adjust_priority(&"b", 0.5);
        assert_eq!(queue.next(), Some("b"));
    }
}
