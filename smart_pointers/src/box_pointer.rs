/*
    Box pointers are pretty straightforward.
    They allow the developer to store data on the heap instead of the stack.
    The stack is only used for storing the pointer.

    Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    But they don’t have many extra capabilities either.
    You’ll use them most often in these situations:
     - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.
     - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
     - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

*/

pub fn run() {
    println!("=== Storing data on heap ===");
    storing_data_on_heap();

    println!("=== Recursive data types ===");
    recursive_data_types();
}

// ==================== Storing Data on the Heap with Box<T> ====================
fn storing_data_on_heap() {
    let b = Box::new(5); // Note that the variable can't be named box, because that's a reserved keyword.
    println!("box = {}", b);
}

// ==================== Recursive Data Types ====================

/* This would not work:

enum LinkedList {
    Cons(i32, LinkedList),
    End,
}

Because the size of LinkedList would be unknown at compile time.
The compiler resolves the amount of memory it needs to allocate for a type by looking at its variants.
In this case, the variants are Cons and End.
Cons contains an i32 and another LinkedList.
This leads to an infinite loop and thus the compiler can't determine the size of LinkedList.
Instead, we can use a Box pointer.
*/

enum LinkedList {
    Cons(i32, Box<LinkedList>), // By doing this, the infinite loop is broken and the compiler can determine the size of LinkedList.
    End,
}

use LinkedList::{Cons, End};

fn recursive_data_types() {
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("This works!");
}

// Quintessence: You likely won't be using Box pointers very often, but they're good to know about if you have recursive data types.
