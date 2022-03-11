
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    next: Option<Rc<RefCell<Node>>>,
    value: i32,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            next: None,
            value, 
        }
    }
}

#[derive(Debug)]
pub struct LinkedList {
    front: Option<Rc<RefCell<Node>>>,
    rear: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    pub fn empty() -> LinkedList {
        LinkedList {
            front: None,
            rear: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.front {
            None => true,
            _ => false,
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        match self.front.clone() {
            Some(b) => {
                self.front = b.borrow_mut().next.take();
                if self.front.is_none() {
                    self.rear = None;
                }
                Some(b.borrow().value.clone())                            
            },
            _ => return None,
        }
    }

    pub fn enqueue(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(Node::new(elem)));
        let node_ref = node.clone();
        match self.rear.take() {
            None => {
                self.rear = Some(node);
                self.front = Some(node_ref);
            }
            Some(b) => {
                b.borrow_mut().next = Some(node);
                self.rear = Some(node_ref);
            }

        }                 
    }
    
    pub fn print(&self) {
        let node_number = 1;
        match self.front.clone() {
            Some(b) => {
                let v = b.borrow().value.clone();
                print!("(Node:{} = {})", node_number, v);
                self.print_list(b.borrow().next.clone(), node_number + 1);
                println!("");
            },
            _ => println!("List empty!"),
        };
    }

    pub fn print_list(&self, node: Option<Rc<RefCell<Node>>>, node_number: i32) {
        match node {
            Some(b) => {
                let v = b.borrow().value.clone();
                print!(" -> (Node:{} = {})", node_number, v);
                self.print_list(b.borrow().next.clone(), node_number + 1);
            },
            _ => return
        }
    }
}