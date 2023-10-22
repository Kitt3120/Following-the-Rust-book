/*
    The Send and Sync traits are just a marker traits that indicate when a type is safe to share between threads and to be used in concurrent contexts.
    This means that Send and Sync actually do not require the developer to implement any methods or functionality.
    They are just added to types to make them work in multi-threaded contexts.
    Otherwise, code would not compile in those cases.
*/

use std::{
    cell::RefCell,
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

pub fn run() {
    println!("=== Send and Sync Traits ===");
    send_and_sync_traits();
}

// As an example, let's take this thread-unsafe implementation of a switch:
#[allow(dead_code)]
struct UnsafeSwitch {
    state: RefCell<bool>, // We're using a RefCell here to allow us to use interior mutability just as when we use a Mutex in the SafeSwitch struct.
}

#[allow(dead_code)]
impl UnsafeSwitch {
    fn new() -> UnsafeSwitch {
        UnsafeSwitch {
            state: RefCell::new(false),
        }
    }

    fn flip(&self) {
        let mut state = self.state.borrow_mut();
        *state = !*state;
    }

    fn state(&self) -> bool {
        *self.state.borrow()
    }
}

/*
    This struct does not have the Send and Sync traits, so the compiler knows that it is not safe to share between threads.

    Imagine this struct is used in a multi-threaded context.
    If two threads try to call flip() at the same time, the switch ends up in an unpredictable state.

    Rust does not allow such a scenario to happen.
    At compile time.
*/

// Now, let's make a thread-safe version of this switch:
#[allow(dead_code)]
struct SafeSwitch {
    state: Mutex<bool>,
}

#[allow(dead_code)]
impl SafeSwitch {
    fn new() -> SafeSwitch {
        SafeSwitch {
            state: Mutex::new(false),
        }
    }

    fn flip(&self) {
        let mut state = self.state.lock().unwrap();
        *state = !*state;
    }

    fn state(&self) -> MutexGuard<bool> {
        self.state.lock().unwrap()
    }
}

/*
    As we implemented the SafeSwitch struct using a Mutex, it is now thread-safe and suitable to be marked as Send and Sync.
    Note that we need to declare this implementation as unsafe, as the Sync trait is unsafe to implement.
*/
unsafe impl Send for SafeSwitch {}
unsafe impl Sync for SafeSwitch {}

// Now the compiler will allow us to use this struct in a multi-threaded context.
fn send_and_sync_traits() {
    println!("Creating a new thread-safe switch");
    let switch = SafeSwitch::new(); // Try changing this to UnsafeSwitch::new() to see the compiler error!

    println!("Creating two arcs to the switch");
    let arc1 = Arc::new(switch);
    let arc2 = Arc::clone(&arc1);

    println!("Creating a new thread that flips the switch 100 times");
    let second_thread = thread::spawn(move || {
        for _i in 0..100 {
            arc2.flip();
        }

        println!("Second thread finished flipping the switch 100 times");
    });

    println!("Flipping the switch 101 times on the main thread");
    for _i in 0..101 {
        arc1.flip();
    }

    println!("Main thread finished flipping the switch 101 times");

    println!("Waiting for the second thread to finish");
    second_thread.join().unwrap();

    println!("Done. The switch is now: {}!", arc1.state());
}

/*
    In conclusion, the Rust compiler prevents the developer from using thread-unsafe code in a multi-threaded context.
    When implementing thread-safe code, the developer has to explicitly mark the code as thread-safe by adding the Send and Sync traits.
    This way, the Rust compiler knows which code is safe to use in a multi-threaded context.
*/
