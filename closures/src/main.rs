fn main() {
    println!("Closures without input parameters");
    closures_without_input_parameters();
    println!("\nClosures with input parameters");
    closures_with_input_parameters();
    println!("\nClosures with return values");
    closures_with_return_values();
    println!("\nFunctions with closures");
    functions_with_closures();
}

fn closures_without_input_parameters() {
    let mut text = String::from("Hello World");

    let closure_immutable_borrow = || {
        println!("Immutable Borrow: {}", text);
    };
    closure_immutable_borrow();

    let mut closure_mutable_borrow = || {
        text.push_str("!");
        println!("Mutable Borrow: {}", text);
    };
    closure_mutable_borrow();

    let closure_move = move || {
        println!("Move: {}", text);
    };
    closure_move();

    // This would fail to compile because the closure moved the value
    //println!("Text: {}", text)
}

fn closures_with_input_parameters() {
    let mut text = String::from("Hello World");

    let closure_immutable_borrow = |text| {
        println!("Immutable Borrow: {}", text);
    };
    closure_immutable_borrow(&text);

    let closure_mutable_borrow = |text: &mut String| {
        text.push_str("!");
        println!("Mutable Borrow: {}", text);
    };
    closure_mutable_borrow(&mut text); // Notice how the mut declaration travelled from the closure definition to reference

    let closure_move = move |text| {
        println!("Move: {}", text);
    };
    closure_move(text);

    // This would fail to compile because the closure moved the value
    //println!("Text: {}", text)
}

fn closures_with_return_values() {
    let generate_string = || -> String { String::from("Hello World") };
    let text = generate_string();
    println!("Text: {}", text);

    let generate_string_from = |text: &str| -> String { String::from(text) };
    let text = generate_string_from("Hello World");
    println!("Text: {}", text);
}

fn functions_with_closures() {
    let text = String::from("Hello World!");
    let print_text = || {
        println!("{}", text);
    };

    closure_function(print_text);
}

fn closure_function(run: impl FnOnce()) {
    run();
}
