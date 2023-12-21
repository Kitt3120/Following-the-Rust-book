/*
    This code is taken from the Rust Language Book, as I am too lazy to write even more examples myself. :p
    Also, the examples are pretty good! Couldn't have done it better myself.

    Below, I gathered patterns which are all valid.
*/

pub fn run() {
    println!("==== Matching literals ====");
    matching_literals();

    println!();

    println!("==== Matching named variables ====");
    matching_named_variables();

    println!();

    println!("==== Matching multiple patterns ====");
    matching_multiple_patterns();

    println!();

    println!("==== Matching range of values ====");
    matching_range_of_values();

    println!();

    println!("==== Destructing structs ====");
    destructing_structs();

    println!();

    println!("==== Destructing enums ====");
    destructing_enums();

    println!();

    println!("==== Destructing nested enums ====");
    destructing_nested_enums();

    println!();

    println!("==== Destructing structs and tuples ====");
    destructing_structs_and_tuples();

    println!();

    println!("==== Ignoring values in patterns ====");
    ignoring_values_in_patterns();

    println!();

    println!("==== Ignoring parts of value with nested underscore ====");
    ignoring_parts_of_value_with_nested_underscore();

    println!();

    println!("==== Ignoring an unused variable with underscore ====");
    ignoring_an_unused_variable_with_underscore();

    println!();

    println!("==== Ignoring remaining parts with dotdot ====");
    ignoring_remaining_parts_with_dotdot();

    println!();

    println!("==== Using dotdot to get first and last ====");
    using_dotdot_to_get_first_and_last();

    println!();

    println!("==== Match guards ====");
    match_guards();

    println!();

    println!("==== Match guard bindings ====");
    match_guard_bindings();
}

// Prints: "one"
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
}

// Prints: "Matched, y = 5"
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
}

// Prints: "one or two"
fn matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Prints: "one through five"
fn matching_range_of_values() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

/*
    Prints:
    Point: (0, 7)
    x = 0, y = 7
    On the y axis at 7
*/
fn destructing_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    println!("Point: {}", p);
    println!("x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

// Prints: "Change the color to red 0, green 160, and blue 255"
fn destructing_enums() {
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// Prints: "Change the color to hue 0, saturation 160, and value 255"
fn destructing_nested_enums() {
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

// Prints: "feet: 3, inches: 10, x: 5, y: -20"
fn destructing_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 5, y: -20 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
}

// Prints: "This code only uses the y parameter: 4"
fn ignoring_values_in_patterns() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

/*
    Prints:
    Can't overwrite an existing customized value
    setting is Some(5)
*/
fn ignoring_parts_of_value_with_nested_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// Prints: "y = 10"
fn ignoring_an_unused_variable_with_underscore() {
    let _x = 5; // x is never used, but it is not marked as unused and will not cause a warning by the compiler
    let y = 10;
    println!("y = {}", y);
}

// Prints: "x is 0"
fn ignoring_remaining_parts_with_dotdot() {
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

// Prints: "First: 2, last: 32"
fn using_dotdot_to_get_first_and_last() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("First: {}, last: {}", first, last),
    }
}

// Prints: "Less than five: 4"
fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("A number: {}", x),
        None => println!("Nothing!"),
    }
}

// Prints: "Found an id in range: 5"
fn match_guard_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
