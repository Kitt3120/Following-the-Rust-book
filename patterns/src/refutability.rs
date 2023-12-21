/*
    In Rust, patterns can be refutable or irrefutable.
    Irrefutable means that the pattern will match for any possible value.
    Refutable means that the pattern might fail to match for some possible value.

    For example, the pattern x is irrefutable because it will match any given value.
    However, the pattern Some(x) is refutable because it will only match Some values that contain a value.

    While the let statements and for loops only accept irrefutable patterns,
    the if let and while let expressions accept both refutable and irrefutable patterns.

    When, by accident, a refutable pattern is used where an irrefutable pattern is required,
    the compiler will give a warning.

    But most of the time, you don't have to think about all this.
    It will just make sense and you will use the right pattern for the right situation unconsciously.
    This is only something to keep in mind when you get a warning or error from the compiler, so you know what it is talking about.
*/

pub fn run() {
    println!("===== The let statement =====");
    let_statement();

    println!();

    println!("===== The if let statement =====");
    if_let_statement();

    println!();

    println!("===== The match statement =====");
    match_statement();
}

// Let statements require irrefutable patterns.
fn let_statement() {
    // let Some(x) = Some(1); // This won't compile, because Some(x) is refutable, it might fail to match some possible value.
    let x = Some(1); // This is fine, because x is irrefutable.
    let (y, z) = (1, 2); // This is fine, because (y, z) is irrefutable.

    println!("x: {:?}, y: {}, z: {}", x, y, z);
}

// The if let statement accepts both refutable and irrefutable patterns.
fn if_let_statement() {
    let x = Some(1);
    if let Some(y) = x {
        println!("y: {}", y);
    }

    // However, the compiler will give a warning if an irrefutable pattern is used.
    if let y = x {
        println!("y: {:?}", y);
    }
}

// The match statement accepts only refutable patterns for its arms, except for the last arm, which accepts an irrefutable pattern.
fn match_statement() {
    let x: Option<i32> = None;
    match x {
        Some(y) => println!("y: {}", y),
        x => println!("x: {:?}", x),
    }
}
