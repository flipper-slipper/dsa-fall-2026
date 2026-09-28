/// A binary min heap, where the item with the lowest priority is always at
/// the root.
pub struct MinHeap<T> {
    /// The heap stored as an array of (item, priority) pairs: the children of
    /// index `i` are at `2 * i + 1` and `2 * i + 2`, and its parent is at
    /// `(i - 1) / 2`.
    items: Vec<(T, f64)>,
}

impl<T: PartialEq> MinHeap<T> {
    /// Creates an empty heap.
    pub fn new() -> Self {
        MinHeap { items: Vec::new() }
    }

    /// Returns `true` if the heap is empty and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Adds `item` to the heap with priority `priority`.
    pub fn insert(&mut self, item: T, priority: f64) {
        // Add to the next open spot, then move it up to restore the heap order
        self.items.push((item, priority));
        self.sift_up(self.items.len() - 1);
    }

    /// Removes and returns the item with the lowest priority, or `None` if the
    /// heap is empty.
    pub fn pop_min(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }

        // Move the smallest item to the end so it can be removed without
        // leaving a gap, then move the new root down to restore the heap order
        let last = self.items.len() - 1;
        self.items.swap(0, last);
        let min = self.items.pop().map(|(item, _)| item);
        self.sift_down(0);
        min
    }

    /// Changes the priority of `item` to `new_priority`.
    pub fn adjust_priority(&mut self, item: &T, new_priority: f64) {
        if let Some(i) = self.items.iter().position(|(e, _)| e == item) {
            self.items[i].1 = new_priority;
            // Only one of these will actually move the item
            self.sift_up(i);
            self.sift_down(i);
        }
    }

    /// Moves the item at index `i` up while its priority is lower than its
    /// parent's.
    fn sift_up(&mut self, mut i: usize) {
        // The root (index 0) has no parent
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.items[i].1 < self.items[parent].1 {
                self.items.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    /// Moves the item at index `i` down while its priority is higher than a
    /// child's.
    fn sift_down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            // Find the smallest of the item and its children (if they exist)
            let mut smallest = i;
            if left < self.items.len() && self.items[left].1 < self.items[smallest].1 {
                smallest = left;
            }
            if right < self.items.len() && self.items[right].1 < self.items[smallest].1 {
                smallest = right;
            }

            // The item is no bigger than its children, so the heap order holds
            if smallest == i {
                break;
            }

            self.items.swap(i, smallest);
            i = smallest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_heap_is_empty() {
        let heap: MinHeap<&str> = MinHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn pop_min_on_empty_returns_none() {
        let mut heap: MinHeap<&str> = MinHeap::new();
        assert_eq!(heap.pop_min(), None);
    }

    #[test]
    fn pop_min_returns_lowest_priority_first() {
        let mut heap = MinHeap::new();
        for (item, priority) in [("e", 5.0), ("b", 2.0), ("d", 4.0), ("a", 1.0), ("f", 2.0), ("c", 3.0)] {
            heap.insert(item, priority);
        }
        let mut order = vec![];
        while let Some(item) = heap.pop_min() {
            order.push(item);
        }
        // "b" and "f" share a priority, so either may come out first
        assert_eq!(order[0], "a");
        assert_eq!(order[3..], ["c", "d", "e"]);
        assert!(heap.is_empty());
    }

    #[test]
    fn lowering_priority_moves_item_to_front() {
        let mut heap = MinHeap::new();
        heap.insert("a", 1.0);
        heap.insert("b", 2.0);
        heap.insert("c", 3.0);
        heap.adjust_priority(&"c", 0.0);
        assert_eq!(heap.pop_min(), Some("c"));
        assert_eq!(heap.pop_min(), Some("a"));
    }

    #[test]
    fn raising_priority_moves_item_to_back() {
        let mut heap = MinHeap::new();
        heap.insert("a", 1.0);
        heap.insert("b", 2.0);
        heap.insert("c", 3.0);
        heap.adjust_priority(&"a", 4.0);
        assert_eq!(heap.pop_min(), Some("b"));
        assert_eq!(heap.pop_min(), Some("c"));
        assert_eq!(heap.pop_min(), Some("a"));
    }

    #[test]
    fn adjusting_missing_item_does_nothing() {
        let mut heap = MinHeap::new();
        heap.insert("a", 1.0);
        heap.adjust_priority(&"z", 0.0);
        assert_eq!(heap.pop_min(), Some("a"));
        assert_eq!(heap.pop_min(), None);
    }
}
