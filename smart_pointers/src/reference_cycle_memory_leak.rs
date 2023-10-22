/*
    Rust is known for its runtime performance and memory safety guarantees.
    Rust, with it's concept of ownership and borrowing, prevents developers to create many kinds of memory errors.
    However, there are some cases where Rust can't prevent memory errors.
    One such case is when we use reference counting pointers in combination with the interior mutability pattern.
    This combination can create reference cycles that will never be cleaned up.
    This is called a memory leak.
*/

use std::{cell::RefCell, rc::Rc};

pub fn run() {
    reference_cycle();
}

#[derive(Debug)]
enum Node {
    Value(i32, RefCell<Rc<Node>>),
    End,
}

impl Node {
    fn next(&self) -> Option<&RefCell<Rc<Node>>> {
        match self {
            Node::Value(_, item) => Some(item),
            Node::End => None,
        }
    }
}

fn reference_cycle() {
    // First, let's create two lists with each containing one item.
    let list_a = Rc::new(Node::Value(1, RefCell::new(Rc::new(Node::End))));
    let list_b = Rc::new(Node::Value(2, RefCell::new(Rc::new(Node::End))));

    // List a: 1 -> End
    // List b: 2 -> End
    println!("a next item = {:?}", list_a);
    println!("b next item = {:?}", list_b);

    // Now, let's use the interior mutability pattern to insert list_b at the end of list_a.
    let mutable_end_of_list_a = list_a.next().unwrap();
    *mutable_end_of_list_a.borrow_mut() = Rc::clone(&list_b);

    // List a: 1 -> 2 -> End
    // List b: 2 -> End
    println!("a next item = {:?}", list_a);
    println!("b next item = {:?}", list_b);

    // Now let's do the same thing but in reverse.
    let mutable_end_of_list_b = list_b.next().unwrap();
    *mutable_end_of_list_b.borrow_mut() = Rc::clone(&list_a);

    // List a: 1 -> 2 -> 1 -> 2 -> 1 -> 2 -> 1 -> 2 -> ...
    // List b: 2 -> 1 -> 2 -> 1 -> 2 -> 1 -> 2 -> 1 -> ...

    // This is a reference cycle.
    // As soon as we try to access either list, we will get a stack overflow.
    // Try it by uncommenting the line below.
    //println!("List a: {:?}", list_a.next());

    // The bad thing about this is that when reference_cycle() goes out of scope,
    // and we drop list_a and list_b, the reference count of the nodes will never reach zero,
    // and the memory will never be freed.
}

/*
    To prevent this, use weak references, as shown in weak_reference.rs.
    This way, the strong_count of the nodes will be able to reach zero.
    And when accessing the nodes through the weak references, we will have to check for expiration.
    Keep this in mind when using reference counting pointers in combination with the interior mutability pattern.
*/
