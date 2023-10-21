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

    Most common smart pointers:

    - String: A UTF-8 encoded, growable string.
    - Vec<T>: A growable array type.
    - Box<T>: A pointer type for heap allocation.
    - Rc<T>: A reference counting type that enables multiple ownership.
    - Ref<T> and RefMut<T>: Smart pointers that enforces the borrowing rules at runtime instead of compile time.
*/

mod box_pointer;
mod deref_trait;

fn main() {
    println!("===== Box Pointer =====");
    box_pointer::run();

    println!("===== Deref Trait =====");
    deref_trait::run();
}