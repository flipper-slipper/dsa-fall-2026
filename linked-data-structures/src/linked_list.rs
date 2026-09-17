use std::ptr;

/// A single element in the list that contains references to next and previous elements.
pub struct Node<T> {
    data: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}


/// A doubly linked list 
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}


impl<T> LinkedList<T> {
    /// Creates an empty list with no head or tail.
    pub fn new() -> Self {
        LinkedList {head: ptr::null_mut(), tail: ptr::null_mut(),}
    }

    /// Inserts `data` at the front of the list, making it the new head.
    pub fn push_front(&mut self, data: T){
        let new = Box::into_raw(Box::new(Node {data, prev: ptr::null_mut(), next: self.head,}));

        unsafe {
            if self.head.is_null() {
                // The list is empyty, and the new node is also the tail
                self.tail = new;
            }
            else {
                (*self.head).prev = new;
            }
        }
        self.head = new;
    }

    /// Inserts `data` at the back of the list, making it the new tail.
    pub fn push_back(&mut self, data: T){
        let new = Box::into_raw(Box::new(Node {data, prev: self.tail, next: ptr::null_mut(),}));

        unsafe {
            if self.tail.is_null() {
                // The list is empty, and the new node is also the head
                self.head = new;
            }
            else {
                (*self.tail).next = new;
            }
        }
        self.tail = new;
    }

    /// Removes and returns the front element, or `None` if the list is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            // Take ownership of the head node back from the raw pointer
            let old = Box::from_raw(self.head);
            self.head = old.next;

            if self.head.is_null() {
                // The list is now empty, so there is no tail either
                self.tail = ptr::null_mut();
            }
            else {
                (*self.head).prev = ptr::null_mut();
            }
            Some(old.data)
        }
    }

    /// Removes and returns the back element, or `None` if the list is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            // Take ownership of the tail node back from the raw pointer
            let old = Box::from_raw(self.tail);
            self.tail = old.prev;

            if self.tail.is_null() {
                // The list is now empty, so there is no head either
                self.head = ptr::null_mut();
            }
            else {
                (*self.tail).next = ptr::null_mut();
            }
            Some(old.data)
        }
    }

    /// Returns a reference to the front element, or `None` if the list is empty.
    pub fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        }
        unsafe { Some(&(*self.head).data) }
    }

    /// Returns a reference to the back element, or `None` if the list is empty.
    pub fn peek_back(&self) -> Option<&T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe { Some(&(*self.tail).data) }
    }

    /// Reverses the list in place by swapping head and tail, and each previous and next reference. This is O(n) with space usage O(1)--better than copying into a new stack
    pub fn reverse(&mut self) {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                std::mem::swap(&mut (*cur).prev, &mut (*cur).next);
                // prev now holds what used to be next, so this walks forward
                cur = (*cur).prev;
            }
        }
        std::mem::swap(&mut self.head, &mut self.tail);
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }
}

/// This is important to add to prevent memory leaking.
impl<T> Drop for LinkedList<T> {
    /// Frees every remaining node when the list goes out of scope.
    fn drop(&mut self) {
        // Free every node we handed out with Box::into_raw
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_null());
        assert!(list.tail.is_null());
    }

    #[test]
    fn push_front_adds_to_head() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&1));
    }

    #[test]
    fn push_back_adds_to_tail() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&2));
    }

    #[test]
    fn pop_front_removes_from_head() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn pop_back_removes_from_tail() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_front_returns_head_without_removing() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek_front(), None);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_front(), Some(&1));
    }

    #[test]
    fn peek_back_returns_tail_without_removing() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek_back(), None);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.peek_back(), Some(&2));
        assert_eq!(list.peek_back(), Some(&2));
    }

    #[test]
    fn is_empty_tracks_contents() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
        list.pop_front();
        assert!(list.is_empty());
    }
}
