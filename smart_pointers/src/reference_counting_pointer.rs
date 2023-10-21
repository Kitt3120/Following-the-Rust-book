/*
    In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
    However, there are cases when a single value might have multiple owners
    In those cases, you can use a reference counting type, which keeps track of the number of references to a value.
    The value will be cleaned up once it has no owners.
*/

use std::rc::Rc;

pub fn run() {
    println!("=== Box Pointer ===");
    using_box_pointer();

    println!("=== Reference Counting Pointer ===");
    using_reference_counting_pointer();
}

// Using the struct from the box_pointer example:;
enum LinkedList {
    Cons(i32, Box<LinkedList>),
    End,
}

// We wouldn't be able to create a list a that is contained in another list b and another list c, so a is shared between b and c.

fn using_box_pointer() {
    let a = LinkedList::Cons(5, Box::new(LinkedList::Cons(10, Box::new(LinkedList::End))));
    let _b = LinkedList::Cons(3, Box::new(a));
    //let c = LinkedList::Cons(4, Box::new(a)); // Doesn't work, a is moved
    println!("Doesn't work! :(");
}

// So let's use a reference counting pointer instead:
enum CoolerLinkedList {
    Cons(i32, Rc<CoolerLinkedList>),
    End,
}

fn using_reference_counting_pointer() {
    let a = Rc::new(CoolerLinkedList::Cons(
        5,
        Rc::new(CoolerLinkedList::Cons(10, Rc::new(CoolerLinkedList::End))),
    ));
    println!("Reference count of a: {}", Rc::strong_count(&a));

    let _b = CoolerLinkedList::Cons(3, Rc::clone(&a));
    println!("Reference count of a: {}", Rc::strong_count(&a));

    let _c = CoolerLinkedList::Cons(4, Rc::clone(&a));
    println!("Reference count of a: {}", Rc::strong_count(&a));

    println!("Works! :)");

    /*
        We're using Rc::clone instead of a.clone() because Rc::clone doesn't make a deep copy of all the data like clone does.
        The call to Rc::clone only increments the reference count, which doesn't take much time.

        When looking for performance problems in your Rust code, you can be assured that calls to Rc::clone aren't expensive.
        Instead, you can focus on the .clone() calls that do deep copies.
    */
}

/*
    Note that Rc<T> is only for use in single-threaded scenarios.
    Handling concurrency is a topic explained in project "concurrency" of this repo.
    Also, Rc<T> is only for use in immutable scenarios.
    If you want multiple mutable references, see ref_cell_pointer.rs.
*/
