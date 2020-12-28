pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    /// Push an item onto the stack
    /// also make sure the stack isn't larger than 16
    pub fn push(&mut self, v: u16) {
        if self.stack.len() == 16 {
            panic!("Stack overflow!")
        }
        self.stack.push(v);
    }

    /// Pop an item from the stack
    /// automatically unwrap to make my life easier
    pub fn pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    /// Wrapper around the Vec.len() function
    pub fn len(&mut self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();

        // Push an item to the stack
        s.push(1);

        assert_eq!(1, s.len()); // Length should be 1
        assert_eq!(1, s.pop()); // 1 should be popped off the stack
        assert_eq!(0, s.len()); // The length should now be 0
    }
}
