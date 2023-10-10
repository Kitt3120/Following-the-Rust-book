pub fn introduce() {
    let longer_string = String::from("Long string!");
    let shorter_string = String::from("string");

    let longest = longest(longer_string.as_str(), shorter_string.as_str());

    println!("The longest string is {}", longest);
}

/*
    Lifetime annotations don't change how long any of the references live.
    It describes how the lifetimes of multiple references relate to each other.
    In this example, we indicate that string slice references x and y have the same lifetime 'a.
    That is the same as the lifetime of the return value.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
