/*
    Raw pointers are one of the additions one has access to when using unsafe Rust.
    They are similar to references, but are different in a few ways.

    Raw pointers are allowed to ignore the borrowing rules by having both
    immutable and mutable pointers or multiple mutable pointers to the same location.

    Raw pointers also aren't guaranteed to point to valid memory.
    They are allowed to be null.

    In addition to that, because they circumvent the borrowing rules, there is no automatic cleanup of memory.

    One use case of raw pointers is when interfacing with other languages, such as C.
*/

pub fn run() {
    println!("Now entering unsafe territory!");
    println!();

    unsafe {
        let mut num = 5;

        let r1 = &num as *const i32; // immutable raw pointer
        let r2 = &mut num as *mut i32; // mutable raw pointer

        println!("Value of num is {}", num);
        println!("Value of what r1 is pointing to: {}", *r1);
        println!("Value of what r2 is pointing to: {}", *r2);

        println!();

        println!("Setting a new value by dereferencing r2...");
        *r2 = 10;

        println!();

        println!("Value of num is {}", num);
        println!("Value of what r1 is pointing to: {}", *r1);
        println!("Value of what r2 is pointing to: {}", *r2);

        //*r1 = 20; // This would cause a compiler error, as r1 is an immutable raw pointer

        println!();

        println!("Address of num is: {:p}", &num);
        println!("r1 points to: {:p}", r1);
        println!("r2 points to: {:p}", r2);
        println!("Value of num is {}", num);
        println!("Address of r1 is: {:p}", &r1);
        println!("Address of r2 is: {:p}", &r2);
    }

    println!("Now leaving unsafe territory!");

    println!();

    println!("But wait! We can still create raw pointers!");

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("We just can't dereference them outside of an unsafe block!");

    println!();

    println!("Address of num is: {:p}", &num);
    println!("r1 points to: {:p}", r1);
    println!("r2 points to: {:p}", r2);
    println!("Address of r1 is: {:p}", &r1);
    println!("Address of r2 is: {:p}", &r2);
    //println!("Value of what r1 is pointing to: {}", *r1); // This would cause a compiler error
    println!("Value of what r1 is pointing to: {}", unsafe { *r1 }); // But you can break down unsafe blocks even this small!
    println!("Value of what r2 is pointing to: {}", unsafe { *r2 });
}
