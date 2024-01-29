struct MyQueue {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { stack: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut tmp_stack = vec![];
        let mut tmp = 0;
        loop {
            if self.stack.is_empty() {
                break;
            }
            tmp = self.stack.pop().unwrap();
            tmp_stack.push(tmp);
        }
        tmp_stack.pop();
        loop {
            if tmp_stack.is_empty() {
                break;
            }
            self.stack.push(tmp_stack.pop().unwrap());
        }
        tmp
    }

    fn peek(&mut self) -> i32 {
        let mut tmp_stack = vec![];
        let mut tmp = 0;
        loop {
            if self.stack.is_empty() {
                break;
            }
            tmp = self.stack.pop().unwrap();
            tmp_stack.push(tmp);
        }
        loop {
            if tmp_stack.is_empty() {
                break;
            }
            self.stack.push(tmp_stack.pop().unwrap());
        }
        tmp
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    println!("{}", queue.peek());
    println!("{}", queue.pop());
    println!("{}", queue.empty());
}
