/*
    A mutex is an operating-system-level construct that allows a thread to lock a specific resource.
    While the resource is locked, no other thread can access it until the lock is released.
    Another thread can block and wait for the lock to become available again.
*/

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn run() {
    println!("=== Mutexes ===");
    mutexes();

    println!("=== Mutexes Multithreaded ===");
    mutexes_multithreaded();

    println!("=== Mutexes with Arc ===");
    mutexes_with_arc();
}

fn mutexes() {
    let data = 5;
    let mutex = Mutex::new(data); // Ownership of data is moved into the mutex

    {
        println!("Acquiring lock...");
        let mut num = mutex.lock().unwrap();
        /*
           Lock the mutex and get a smart pointer to the data inside.
           This line blocks the thread until the lock is available.
           In a real program, you would handle the error case here.
        */

        println!("Lock acquired");

        println!("Changing mutex data");
        *num = 6; // Change the data inside the mutex

        println!("MutexGuard goes out of scope now, Mutex will be unlocked");
    } // The lock is released here when the MutexGuard goes out of scope

    println!("Mutex data: {:?}", mutex);
}

/*
    Note that we can't use this in multithreaded scenarios, as we would have to move ownership of the mutex to another thread.
    Then it wouldn't be available in other threads anymore.

    We learned about the Rc<T> type before, which allows us to have multiple owners of a value.
    This could help us in this case, but we also learned that Rc<T> is not thread-safe, so we can't use it in multithreaded scenarios.
    Rc<T> can't be moved into threads because it does not implement the Send trait.
    It is also not safe to be accessed from multiple threads at the same time because it does not implement the Sync trait.
    This is because when Rc counts up or down its reference count, it does so using a primitive data type that is not atomic.
    This is a good choice made by the Rust team, because atomic operations are more expensive than non-atomic ones,
    so they should only be used when necessary.
*/

fn mutexes_multithreaded() {
    let data = 5;
    let mutex = Mutex::new(data);
    let reference_counting_pointer = Rc::new(mutex);

    /*
    let rc_clone = Rc::clone(&reference_counting_pointer);
    thread::spawn(move || {
        let mut num = rc_clone.lock().unwrap();
        *num = 6;
    });
    */

    println!("Above code wouldn't compile, try it out!");
    println!("Mutex data: {:?}", reference_counting_pointer);
}

/*
    This is where Arc<T> comes in.
    Arc behaves like Rc, but it is atomic, therefore thread-safe and implements the Send trait.
    Under the hood, instead of using a primitive data type to count references, it uses atomic operations.
    This makes it more expensive, but also thread-safe.
    Note that the thread-safety of Arc<T> is only guaranteed for the reference count, not for the data inside.
    That's why we still need to use a Mutex<T> to ensure thread-safety for the data inside.
*/

fn mutexes_with_arc() {
    let data = 5; // Note: immutable!
    let mutex = Mutex::new(data);
    let atomic_reference_counting_pointer = Arc::new(mutex);
    let arc_clone = Arc::clone(&atomic_reference_counting_pointer);

    thread::spawn(move || {
        println!("Second thread: Acquiring lock...");
        let mut num = arc_clone.lock().unwrap();

        println!("Second thread: Waiting 3 seconds to simulate work while holding the lock...");
        thread::sleep(std::time::Duration::from_secs(3));

        println!("Second thread: Changing mutex data from {} to 6", num);
        *num = 6; // Interior mutability!

        println!("Second thread: MutexGuard and arc_clone go out of scope now. Mutex will be unlocked and strong reference of arc_clone will be released");
    });

    println!("Main thread: Waiting 100 milliseconds to make sure the second thread has acquired the lock...");
    thread::sleep(std::time::Duration::from_millis(100));

    println!("Main thread: Acquiring lock...");
    let num = atomic_reference_counting_pointer.lock().unwrap();
    println!("Main thread: Got lock! Mutex data is {}", num);

    println!("Main thread: Mutex and atomic_reference_counting_pointer go out of scope now. Mutex will be unlocked and strong reference of atomic_reference_counting_pointer will reach strong reference count of 0 and be freed.");
}

/*
    Note that if you are doing simple numerical operations,
    there are types simpler than Mutex<T> types provided by the std::sync::atomic module of the standard library.
    These types provide safe, concurrent, atomic access to primitive types.
    We chose to use Mutex<T> with a primitive type for this example so we could concentrate on how Mutex<T> works.

    Also note that, just as RefCell<T>, Mutex<T> provides interior mutability, as shown in line 101.
    This means that we can share a Mutex between threads using Arc<T>, acquire a lock on it and mutate the data inside.
    By doing this, we guarantee thread-safety.
*/
