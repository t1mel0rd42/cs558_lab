use std::fmt::Display;

const N:usize = 8;

#[derive(Debug)]
pub struct MyQueue<T> {
    values: [Option<T>;N+1],
    front: usize,
    rear: usize,
}

impl<T: Copy + Display> MyQueue<T> {
    pub fn empty() -> MyQueue<T> {
        MyQueue {
            values: [None;N+1],
            front: 0,   
            rear: 0,
        }
    }
                                                                                                                                                    
    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.front == self.rear {
            return None
        }
        else {
            let value = self.values[self.front];
            self.front = (self.front + 1) % (N+1); 
            return value
        }
    }

    pub fn enqueue(&mut self, value:T) {
        if (self.rear + 1) % (N+1) == self.front {
            assert_eq!((self.rear + 1) % (N+1), self.front);
            println!("Queue full");
        }
        else {
            self.values[self.rear] = Some(value);
            self.rear = (self.rear + 1) % (N+1);
            println!("Added: {}  Front: {}  Rear: {}", value, self.front, self.rear);

        }
    }
}