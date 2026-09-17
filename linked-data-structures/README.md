# Linked Data Structures
This folder contains implementations of a Doubly Linked List, and a Stack and Queue that uses the Doubly Linked List. This problem is non-trivial in Rust due to memory safety rules. Rust prohibits two variables from containing mutable references to each other. 

There are a few ways to do this, but the `LinkedList` structure in the standard library gets around the issue by using raw pointers, which is what I also decided to use. Because I'm dealing with raw memory in rust, the code is wrapped in an `unsafe` block, and memory is manually scrutinized to make sure there are no memory leaks. For instance, I had to implement the `Drop` trait. 

Run with `cargo test`. 

## Practice Problems with Stacks and Queues
### Reversing Elements in Stack 
I implemented this in `stack.rs`

Swap the head and tail nodes, then swap the prev and next nodes for each node, taking care of the head and tail cases. Complexity is O(n) for iterating through the Stack once, and space is O(1) for holding a copy to one of the old head/tails when swapping them around.
### Valid Parenthesis
Iterate through each character in the string. For every opening character, push its complementary terminating character onto a stack. For each terminating character, pop the stack; if the stack is empty or the popped character differs from the current one, the parentheses are invalid. The string is valid only if the stack is empty once every character has been processed.

### Copy Stack Problem

In implementation reality I would have access to the underlying linked list, so I could just walk along it and rebuild the stack in a new list. That's one pass and the original is never touched. In Rust this is what implementing `Clone` for the stack would do.

If I only have access to the public interface methods (`pop`, `push`, `peek`, `is_empty`) and one queue, I would pop everything into the queue and then dequeue everything back onto the stack. Since popping gives me the elements top-first, this leaves the stack reversed. Then I do it one more time: pop everything into the queue, and while dequeuing, push each element onto both the original stack and a new one. Two reversals cancel out, so the original is back to normal and the new stack is a copy of it. This is O(n) time and O(n) extra space for the queue.