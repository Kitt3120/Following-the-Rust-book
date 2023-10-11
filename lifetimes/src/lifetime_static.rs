/*
    A static lifetime means that the reference can live for the entire duration of the program.
    For example, all string literals have the 'static lifetime
    because the text of the string is stored directly in the program's binary
    and is loaded into memory when the program starts.
    Therefore, string literals are available for the entire duration of the program.
*/

// A function that only accepts a statically allocated string
fn print_static_string(string: &'static str) {
    println!("{}", string);
}

pub fn introduce() {
    //let static_string = "I'm a static string"; // This string is statically allocated
    let static_string: &'static str = "I'm a static string"; // And can be explicitly annotated as such
    print_static_string(static_string); // Works!

    let string = String::from("I'm a string"); // This string is dynamically allocated
                                               //print_static_string(&string); // This would not work, because the string is not statically allocated
}
