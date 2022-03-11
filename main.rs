#![allow(dead_code)]
mod queue;
mod linkedlist;

use queue::MyQueue;
use linkedlist::LinkedList;

         const QUEUE_INT_TEST: bool = true;
      const QUEUE_STRING_TEST: bool = true;
   const LINKEDLIST_INT_TEST: bool = true;
//const LINKED_LIST_STRING_TEST: bool = false;

fn main() {
    if QUEUE_INT_TEST == true {
        let mut q_int: MyQueue<i32> = MyQueue::empty();
        test_queue_int(&mut q_int);
    }
    if QUEUE_STRING_TEST == true {
        let mut q_str: MyQueue<&str> = MyQueue::empty();
        test_queue_str(&mut q_str);
    }
    if LINKEDLIST_INT_TEST == true {
        let mut ll: LinkedList = LinkedList::empty();
        test_ll(&mut ll);
    }
}

fn test_ll(ll: &mut LinkedList) {
    println!("");
    println!("***** LINKED LIST - INT TEST *****");
    println!("Is Empty? {}", ll.is_empty());
    match ll.dequeue() {
        Some(v) => println!("Popped: {}",v),
        _ => println!("Popped: None"),
    }
    ll.print();
    println!("Enqueue 4,5,6");
    ll.enqueue(4);
    ll.enqueue(5);
    ll.enqueue(6);
    ll.print();
    println!("Is Empty? {}", ll.is_empty());
    match ll.dequeue() {
        Some(v) => println!("Dequeued: {}",v),
        _ => println!("Dequeued: None"),
    }
    match ll.dequeue() {
        Some(v) => println!("Dequeued: {}",v),
        _ => println!("Dequeued: None"),
    }
    ll.print();
    println!("Enqueue 7");
    ll.enqueue(7);
    ll.print();
    println!("Is Empty? {}", ll.is_empty());
    for _i in 0..6 {
        match ll.dequeue() {
            Some(v) => println!("Dequeued: {}",v),
            _ => println!("Dequeued: None"),
        }
    }
    println!("Is Empty? {}", ll.is_empty());
    ll.print()
}

fn test_queue_str(q: &mut MyQueue<&str>) {
    println!("");
    println!("***** QUEUE - STRING TEST *****");
    println!("Queue is empty: {}", q.is_empty());
    match q.dequeue() {
        Some(v) => println!("Popped: {}",v),
        _ => println!("Popped: None"),
    }
    for _n in 0..5 {
        for _i in 0..5 {
            q.enqueue("DATA!!");
        }
        for _c in 0..3 {
            match q.dequeue() {
                Some(v) => println!("Popped: {}",v),
                _ => println!("Popped: None"),
            }
        }
    }
    println!("Queue is empty: {}", q.is_empty());
    for _i in 0..6 {
        match q.dequeue() {
            Some(v) => println!("Dequeued: {}",v),
            _ => println!("Dequeued: None"),
        }
    }
    println!("Queue is empty: {}", q.is_empty());
}

fn test_queue_int(q: &mut MyQueue<i32>) {
    println!("");
    println!("***** QUEUE - INT TEST *****");
    println!("Queue is empty: {}", q.is_empty());
    match q.dequeue() {
        Some(v) => println!("Popped: {}",v),
        _ => println!("Popped: None"),
    }
    for n in 0..5 {
        for i in 0..5 {
            q.enqueue(n+i);
        }
        for _c in 0..3 {
            match q.dequeue() {
                Some(v) => println!("Popped: {}",v),
                _ => println!("Popped: None"),
            }
        }
    }
    println!("Queue is empty: {}", q.is_empty());
    for _i in 0..6 {
        match q.dequeue() {
            Some(v) => println!("Dequeued: {}",v),
            _ => println!("Dequeued: None"),
        }
    }
    println!("Queue is empty: {}", q.is_empty());
}
