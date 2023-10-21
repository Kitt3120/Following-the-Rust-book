/*
    Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally,
    this action is disallowed by the borrowing rules.
    To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
    Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us.

    We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime,
    even though the compiler can’t guarantee that.
    The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

    Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
    So, what makes RefCell<T> different from a type like Box<T>?
    Recall the borrowing rules:
     - At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
     - References must always be valid.

    With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
    With RefCell<T>, these invariants are enforced at runtime.
    With references, if you break these rules, you’ll get a compiler error.
    With RefCell<T>, if you break these rules, your program will panic and exit.

    The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process,
    and there is no impact on runtime performance because all the analysis is completed beforehand.
    For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

    The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed,
    where they would’ve been disallowed by the compile-time checks.
    Static analysis, like the Rust compiler, is inherently conservative.
    Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem.

    Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program.
    In this way, it’s conservative.
    If Rust accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes.
    However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur.
    The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

    Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios
    and will give you a compile-time error if you try using it in a multithreaded context.
*/

pub fn run() {
    println!("=== Simple Messenger ===");
    trigger_limit();
    println!("Run cargo test for the Mock Messenger!");
}

/*
    Our library will only provide the functionality of tracking how close to the maximum a value is and what the messages should be at what times.
    Applications that use our library will be expected to provide the mechanism for sending the messages.
    The application could put a message in the application, send an email, send a text message, or something else.
    The library doesn’t need to know that detail.
    All it needs is something that implements a trait we’ll provide called Messenger
*/

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    // The lifetime parameter 'a means an instance of LimitTracker can’t outlive the reference in its lifetime parameter.
    // The second parameter T has the trait bound Messenger, which means we can call send on T.
    messenger: &'a T,
    // The LimitTracker struct has a field named messenger that holds a reference to something that implements the Messenger trait.
    // This will hold an instance of a type that implements the Messenger trait to handle sending the warning messages.
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.0 {
            self.messenger
                .send("Urgent warning: You've used up over 75% of your quota!");
        } else {
            self.messenger
                .send("Warning: You've used up over 50% of your quota!");
        }
    }
}

impl Messenger for String {
    fn send(&self, msg: &str) {
        println!("{}: {}", self, msg);
    }
}

fn trigger_limit() {
    let simple_messenger = String::from("Mock Messenger");
    let mut limit_tracker = LimitTracker::new(&simple_messenger, 100);

    limit_tracker.set_value(80);
}

/*
    If we now try to test the LimitTracker struct, we would run into the problem that set_value does not return anything.
    We can’t make an assertion about what happens inside the function because it doesn’t return anything we could check.
    Instead, we have to make our variant of a Messenger that keeps track of the messages it receives.
*/

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    /*

    struct MockMessenger {
        messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, _message: &str) {
            //self.messages.push(String::from(message)); // And here's the problem, this does not work, as self is immutable. Let's use RefCell<T> to solve this.
            println!("This doesn't work!");
        }
    }

    */

    struct BetterMockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl BetterMockMessenger {
        fn new() -> BetterMockMessenger {
            BetterMockMessenger {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for BetterMockMessenger {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(String::from(message)); // Here, we can borrow the messages vector mutably, as provided by RefCell<T>.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = BetterMockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.messages.borrow().len(), 1);
        assert_eq!(
            mock_messenger.messages.borrow()[0],
            "Urgent warning: You've used up over 75% of your quota!"
        );
    }
}
