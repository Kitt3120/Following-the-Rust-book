/*
    Rust has a special type called Never (!) that is used for functions that never return.
    The Never type can't be instantiated, so it is not possible to create a value of type Never.
    This is useful for functions that exit the process or infinite loops.

    The Never type is also useful for Rusts type inference.
    What if a match statement has a match arm that returns an i32 value and another match arm that continues?
    Rust treats continue as a value of type Never, which means that its match arm can't return a value.
    That leaves only the other match arm, which returns an i32 value.
    So the match statement returns an i32 value.
*/

use std::io::{stdin, stdout, Write};

pub fn run() {
    println!("===== Loop example =====");
    loop_example();

    println!();

    println!("===== Match example =====");
    match_example();

    println!();

    println!("===== Panic example =====");
    //_panic_example(); // Enable if you want to see the panic!
}

/*
    Notice that fifteen is inferred to be of type i32 because continue is of type Never.
    Also, by logic, this is the only possible outcome!
*/
fn loop_example() {
    let mut number = 10;
    println!("Starting from {}", number);
    let _fifteen = loop {
        if number == 15 {
            break number;
        } else if number < 15 {
            number += 1;
        } else if number > 15 {
            number -= 1;
        }

        println!("Now at {}", number);
    };
}

fn match_example() {
    print!("Type something: ");
    stdout().flush().unwrap();

    let text = loop {
        let mut input = String::new();
        let result = stdin().read_line(&mut input);
        match result {
            Ok(_num) => break input,
            Err(_) => continue,
        }
    };

    println!("You wrote: {}", text);
}

fn _panic_example() -> ! {
    panic!("This function panics!");
}
