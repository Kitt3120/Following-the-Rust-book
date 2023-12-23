/*
    Similar to closures, functions can also be passed as arguments to other functions.
    Rust supports higher-order functions by using the fn type.

    The use of the fn type is however discouraged opposed to using closures.
    The fn type is still useful when interfacing with code that uses fn, such as C code.
    C accepts functions as arguments but it does not accept closures.

    Below are examples to compare the different syntaxes when passing closures and functions as arguments.
*/

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn run() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    example_closure();
    example_fn();
    example_enum();
}

fn example_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);
}

fn example_fn() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);
}

#[derive(Debug)]
enum Status {
    Value(u32), // Remember that enum variants are initializer functions?
}

fn example_enum() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}
