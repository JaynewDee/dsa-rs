#![allow(unused)]

pub mod linked_lists {
    struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    struct LinkedList {
        head: Option<Box<ListNode>>,
    }

    impl LinkedList {
        pub fn new() -> Self {
            Self { head: None }
        }

        pub fn push_front(&mut self, val: i32) {
            let new_node = Box::new(ListNode {
                val,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        pub fn pop_front(&mut self) -> Option<i32> {
            self.head.take().map(|b| {
                self.head = b.next;
                b.val
            })
        }
    }

    pub fn run() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(1);
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn it_works() {}
    }
}

pub mod stacks {
    pub trait LIFO<T> {
        fn load(self, vals: Vec<T>) -> Self;
        fn push(self, val: T) -> Self;
        fn pop(self) -> Option<T>;
    }

    pub struct Stack<T> {
        values: Vec<T>,
    }

    impl LIFO<String> for Stack<String> {
        fn load(mut self, vals: Vec<String>) -> Self {
            self.values = vals;
            self
        }

        fn push(mut self, val: String) -> Self {
            self.values.push(val);
            self
        }

        fn pop(mut self) -> Option<String> {
            self.values.pop()
        }
    }

    impl Stack<String> {
        pub fn new(capacity: usize) -> Self {
            Self {
                values: Vec::with_capacity(capacity),
            }
        }
    }
}

pub mod queues {
    pub trait FIFO<T> {
        fn load(self, vals: Vec<T>) -> Self;
        fn enqueue(self, val: T) -> Self;
        fn dequeue(self) -> T;
    }

    pub struct Queue<T> {
        values: Vec<T>
    }

    impl FIFO<String> for Queue<String> {
        fn load(mut self, vals: Vec<String>) -> Self {
            self.values = vals;
            self
        }

        fn enqueue(mut self, val: String) -> Self {
            self.values.push(val);
            self
        }

        fn dequeue(mut self) -> String {
            self.values.remove(0)
        }
    }

    impl Queue<String> {
        pub fn new(capacity: usize) -> Self {
            Self {
                values: Vec::with_capacity(capacity)
            }
        }
    } 
}
