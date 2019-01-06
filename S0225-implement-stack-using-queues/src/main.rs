struct MyStack {
    queue: LinkedList<i32>,
}
use std::collections::LinkedList;

/** You can modify the type of `self` for your need. */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        return MyStack {
            queue: LinkedList::new(),
        };
    }
    /** Push element x onto stack. */

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    /** Removes the element on top of the stack and returns that element. */

    fn pop(&mut self) -> i32 {
            return self.queue.pop_back().unwrap();
    }
    /** Get the top element. */

    fn top(&mut self) -> i32 {
        let n = self.queue.pop_back().unwrap();
        self.queue.push_back(n);
        return n;
    }
    /** Returns whether the stack is empty. */

    fn empty(&self) -> bool {
        return self.queue.is_empty();
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    println!("Hello, world!");
}
