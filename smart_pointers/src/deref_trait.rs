/*
    Implementing the Deref trait allows you to customize the behavior of the dereference operator *.
    By implementing Deref, a smart pointer can be treated like a regular reference.
    By implementing Deref in such a way that a smart pointer can be treated like a regular reference,
    you can write code that operates on references and use that code with smart pointers too.
*/

use std::{fmt::Display, ops::Deref};

pub fn run() {
    println!("=== Using a reference ===");
    following_a_reference();

    println!("=== Using MyBox<T> ===");
    using_mybox_good();

    println!("=== Deref Coercion ===");
    deref_coercion();
}

pub fn following_a_reference() {
    let x = 5;
    let y = &x;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("x is 5 is {}", x == 5);
    println!("*y is 5 is {}", *y == 5); // This wouldn't work without the * because y is a reference to x, not x itself.
}

struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox { value }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value // This returns a reference to the value we want to access with the dereference operator.
    }
}

/*
    Note that the return type of the deref method is &T.
    The deref method only returns a reference to the value in the box, as returning the value would lead to a move of ownership.
*/

impl<T> Display for MyBox<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn using_mybox_good() {
    let x = 5;
    let y = MyBox::new(x);

    println!("x = {}", x);
    println!("y = {}", y);
    println!("x is 5 is {}", x == 5);
    println!("*y is 5 is {}", *y == 5);

    // Behind the scenes, Rust actually ran *(y.deref()) for us.
}

/*
    When using a deref-implementing type as an argument in a function and the type doesn't match the type defined in the function signature,
    Rust will automatically use the deref method to coerce the type into the type defined in the function signature, if possible.
    This is called deref coercion.
    It happens automatically and is a convenient feature of Rust.
*/

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn deref_coercion() {
    let b = MyBox::new(String::from("Rust"));
    hello(&b);

    // If Rust did not have deref coercion, we would have to write the following code:
    hello(&(*b)[..]); // (*b) dereferences the MyBox<String> into a String.
                      // Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.
}

/*
    Similar to how you use the Deref trait to override the * operator on immutable references,
    you can use the DerefMut trait to override the * operator on mutable references.

    Rust does deref coercion when it finds types and trait implementations in three cases:
     - From &T to &U when T: Deref<Target=U>
     - From &mut T to &mut U when T: DerefMut<Target=U>
     - From &mut T to &U when T: Deref<Target=U>

    The first two cases are the same as each other except that the second implements mutability.
    The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.
    The second case states that the same deref coercion happens for mutable references.

    The third case is trickier:
    Rust will also coerce a mutable reference to an immutable one.
    But the reverse is not possible: immutable references will never coerce to mutable references.
    Because of the borrowing rules, if you have a mutable reference,
    that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
    Converting one mutable reference to one immutable reference will never break the borrowing rules.
    Converting an immutable reference to a mutable reference would require
    that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
    Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
*/
