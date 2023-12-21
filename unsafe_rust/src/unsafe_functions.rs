/*
    Functions and methods can be declared as unsafe in Rust.
    This enables unsafe Rust for the entire body of the function or method.
    When calling unsafe functions, the caller must also be in an unsafe scope.
*/

pub fn run() {
    unsafe {
        println!("Calling an unsafe function!");
        whoops();
    }
}

unsafe fn whoops() {
    println!("Creating a null pointer");
    let null_pointer = 0 as *const i32;

    println!("Address of null pointer: {:p}", &null_pointer);
    println!("Address that null pointer points to: {:p}", null_pointer);
    //println!("Dereferencing null pointer: {}", *null_pointer); // This will cause a segfault at runtime
}
