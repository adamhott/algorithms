// Leetcode 155. Min Stack

// Note: this code submission on Leetcode is not
// accepted due to a error from serializing. 
// Evalulate later.

//Design a stack that supports push, pop, top, 
//and retrieving the minimum element in constant time.

//Implement the MinStack class:

//MinStack() initializes the stack object.
//void push(int val) pushes the element val onto the stack.
//void pop() removes the element on the top of the stack.
//int top() gets the top element of the stack.
//int getMin() retrieves the minimum element in the stack.
//You must implement a solution with O(1) time complexity 
//for each function.

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min_val = if let Some(&last_min) = self.min_stack.last() {
            std::cmp::min(val, last_min)
        } else {
            val
        };
        self.min_stack.push(min_val);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> Option<i32> {
        self.stack.last().copied()
    }

    fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack_example_1() {
        let mut min_stack = MinStack::new();

        // Corresponds to ["MinStack","push","push","push","getMin","pop","top","getMin"]
        // and [[], [-2], [0], [-3], [], [], [], []]

        // Test stack initialization
        assert_eq!(min_stack.stack.len(), 0);
        assert_eq!(min_stack.min_stack.len(), 0);

        // Test push operations
        min_stack.push(-2);
        assert_eq!(min_stack.stack.last(), Some(&-2));
        assert_eq!(min_stack.min_stack.last(), Some(&-2));

        min_stack.push(0);
        assert_eq!(min_stack.stack.last(), Some(&0));
        assert_eq!(min_stack.min_stack.last(), Some(&-2)); // -2 is still the min

        min_stack.push(-3);
        assert_eq!(min_stack.stack.last(), Some(&-3));
        assert_eq!(min_stack.min_stack.last(), Some(&-3)); // -3 is now the min

        // Test getMin
        assert_eq!(min_stack.get_min(), Some(-3));

        // Test pop
        min_stack.pop();
        assert_eq!(min_stack.stack.last(), Some(&0));
        assert_eq!(min_stack.min_stack.last(), Some(&-2));

        // Test top
        assert_eq!(min_stack.top(), Some(0));

        // Test getMin after pop
        assert_eq!(min_stack.get_min(), Some(-2));
    }
}



