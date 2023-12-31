/*
    A pointer is a general concept for a variable that contains an address in memory.
    This address refers to, or “points at,” some other data.
    The most common kind of pointer in Rust is a reference.
    References are indicated by the & symbol and borrow the value they point to.
    They don’t have any special capabilities other than referring to data, and have no overhead.

    Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
    Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references.
    Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers:
    While references only borrow data, in many cases, smart pointers own the data they point to.

    Smart pointers are usually implemented using structs.
    Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
    The Deref trait allows an instance of the smart pointer struct to behave like a reference,
    so you can write your code to work with either references or smart pointers.
    The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

    Smart Pointers (not thread-safe):
     - Vec<T>: A growable array type.
     - String: A UTF-8 encoded, growable string type.
     - Box<T>: A pointer type for heap allocation.
     - Rc<T>: A reference counting type that enables multiple ownership.
     - Ref<T>, RefMut<T>, RefCell<T>: Smart pointers that enforces the borrowing rules at runtime instead of compile time.
        > When you borrow from a RefCell<T>, you'll get a Ref<T> or a RefMut<T>.

    Smart Pointers (thread-safe):
     - Arc<T>: A reference counting type that enables multiple ownership. The atomic counterpart to Rc<T>. Often used with Mutex<T> to allow access from multiple threads and interior mutability.
     - Mutex<T>: A smart pointer that allows access to an inner value only one thread at a time.

    When to use what:
     - Box<T> for allocating values on the heap. This enables recursive types like a linked list or a tree. Also allows interior mutability.
     - Rc<T> when ownership of a value needs to be shared between multiple owners. Allows for only immutable references. Single-threaded only.
     - RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time. Useful for the interior mutability pattern. Single-threaded only.
        > Combine RefCell<T> with Rc<T> to get multiple owners with interior mutability. Single-threaded only.
     - Arc<T> counterpart to Rc<T>, but atomic and thus thread-safe. Useful for multiple owners from multiple threads.
     - Mutex<T> for mutably accessing data from multiple threads. Useful to synchronize access to data between multiple threads. Also provides interior mutability.
        > Combine Mutex<T> with Arc<T> to get multiple owners with interior mutability from multiple threads.
*/

mod box_pointer;
mod deref_trait;
mod drop_trait;
mod reference_cell_pointer;
mod reference_counting_pointer;
mod reference_cycle_memory_leak;
mod weak_reference;

fn main() {
    println!("===== Box Pointer =====");
    box_pointer::run();

    println!("===== Deref Trait =====");
    deref_trait::run();

    println!("===== Drop Trait =====");
    drop_trait::run();

    println!("===== Reference Counting Pointer =====");
    reference_counting_pointer::run();

    println!("===== Ref Cell Pointer =====");
    reference_cell_pointer::run();

    println!("===== Weak Reference =====");
    weak_reference::run();

    println!("===== Reference Cycle Memory Leak =====");
    reference_cycle_memory_leak::run();
}
