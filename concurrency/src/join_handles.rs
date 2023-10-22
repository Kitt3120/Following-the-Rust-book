use std::{thread, time::Duration};

pub fn run() {
    println!("=== Join Handles ===");

    join_handles();
}

fn join_handles() {
    let thead_join_handle = thread::spawn(|| count_to_10("Thread", Duration::from_millis(5)));
    count_to_10("Main", Duration::from_millis(1));

    // Wait for the thread to finish.
    // Try what happens if you comment out this line.
    thead_join_handle.join().expect("Error joining thread.");
}

fn count_to_10(label: &str, timeout: Duration) {
    for i in 1..11 {
        println!("{} - Count: {}", label, i);
        thread::sleep(timeout);
    }
}

/*
    Removing the join() call will cause the program to exit before the second thread has finished.
    This is because the main thread finishes before the second thread.
    This will cause the program to exit and the second thread will be terminated, no matter if it has finished or not.
    To prevent this, we need to wait for the second thread to finish before the main thread exits.
*/
