/*
    It's possible to declare a function as safe while using unsafe blocks inside it.
    This is useful when you want to provide a safe API around unsafe code.

    Rust allows this because, as a developer, you make sure that the unsafe code is safe.
    It's just that the compiler can't verify it.

    By allowing this, Rust fixes the issue that the unsafe keyword would cascade up the call stack
    and leave all high level code marked as unsafe.
*/

pub fn run() {
    safe_function();
}

unsafe fn unsafe_function() {
    let x = 5;
    let x_ptr = &x as *const i32;

    println!("x is: {}", x);
    println!("x_ptr dereferences to: {}", *x_ptr);
}

fn safe_function() {
    println!("This is a safe function!");
    println!("Yet, we can call unsafe functions inside it.");

    unsafe {
        unsafe_function();
    }
}
