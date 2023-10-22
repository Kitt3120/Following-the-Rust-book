pub fn run() {
    println!("=== Thread Move Closure ===");

    thread_move_closure();
}

fn thread_move_closure() {
    let mut data = Vec::<i32>::new();
    data.push(1);
    data.push(2);

    // When we want to pass data to a thread, we need to move it.
    // Otherwise, Rust wouldn't know how long the thread will.
    // The thread could outlive the data, which would cause a dangling pointer.
    let join_handle = std::thread::spawn(move || {
        println!("Here's the vector: {:?}", data);
    });

    // We can't use the data anymore, because it's been moved to the thread.
    //drop(data);

    join_handle.join().unwrap();
}
