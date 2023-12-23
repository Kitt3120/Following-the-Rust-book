/*
    Copied from the The Rust Programming Language book:

    "Closures are represented by traits, which means you can’t return closures directly.
    In most cases where you might want to return a trait,
    you can instead use the concrete type that implements the trait as the return value of the function.
    However, you can’t do that with closures because they don’t have a concrete type that is returnable.
    You’re not allowed to use the function pointer fn as a return type, for example."


    This means that closures and fn do not implement the Sized trait and their size is unknown at compile time.
    To get around this, the same principle applies as with dynamically sized types and trait objects: Wrap the closure with a pointer.
*/

pub fn run() {
    let closure = returns_closure();
    let result = closure(1);
    println!("The closure returns: {}", result);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
