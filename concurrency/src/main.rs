/*
    Rust wants to solve memory and concurrency problems.
    Initially, the Rust team thought these were two separate problems.
    But when working on the language, they realized that these problems share a lot of the same solutions.

    With the ownership and borrowing system, Rust actually also solves a lot of problems that come with concurrency.
    Many errors that would happen at runtime in other languages are caught at compile time in Rust, thanks to the ownership and borrowing system.

    Therefore, rather than taking time to debug a problem by reproducing it at runtime, Rust helps you fight bugs at compile time.
    Code with errors like race conditions would just not compile in Rust.
    As a result, you can fix those errors before your code is even run, while working on it and still developing it, instead of it shipping to production.
    This also means that you can be more confident that your code will work as expected when it compiles and you run it.

    This is nicknamed the "fearless concurrency" feature of Rust.
    Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

    With Rust, you can do the following while ensuring thread safety:
     - Spawn threads to run multiple pieces of code at the same time.
     - Send a value to another thread using a message passing channel.
     - Share a value between multiple threads using shared state concurrency.
     - Implement thread-safety in your own custom types using the Sync and Send traits from the standard library.
*/

mod channels;
mod join_handles;
mod thread_move_closure;

fn main() {
    println!("===== Join Handles =====");
    join_handles::run();

    println!("===== Thread Move Closure =====");
    thread_move_closure::run();

    println!("===== Channels =====");
    channels::run();
}
