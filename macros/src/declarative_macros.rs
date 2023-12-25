/*
    Let's implement something similar to the vec! macro.
    A declarative macro is defined like a match expression.
    The left side of the => is the pattern, and the right side is the code that should be executed if the pattern matches.
    However, match statements are matched against values, while macros are matched against Rust code structures.
    This makes the pattern syntax different, and more complex.
    A macro can define multiple patterns so it can match different structures and work differently depending on the structure.
    Should a pattern be passed to the macro that doesn't match any of the defined patterns, the compiler will throw an error.
*/

#[macro_export] // Makes the macro available to other crates. Without this, the macro can't be brought into scope.
macro_rules! vec {          // Define a new macro named vec.
    ( $( $x:expr ),* ) => { // The ( $( $x:expr ),* ) expression defines an outer variable $() and an inner variable $x.
        {                   // The inner $x:expr part is a pattern that matches any Rust expression and binds it to $x.
            let mut temp_vec = Vec::new(); // The * indicates that the pattern matches zero or more of whatever precedes the *.
            $(              // The $()* states that the scope within should be repeated for every $x matched, like a for loop.
                temp_vec.push($x); // The $x is replaced with the value matched by the $x pattern for that iteration.
            )*              // The * indicates that this scope should be repeated zero or more times.
            temp_vec        // The macro returns the temp_vec.
        }
    };
}

pub fn run() {
    let v = vec![1, 2, 3];
    /*
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       Resulting code:
       let mut temp_vec = Vec::new();
       temp_vec.push(1);
       temp_vec.push(2);
       temp_vec.push(3);
       temp_vec
    */

    println!("{:?}", v);
}
