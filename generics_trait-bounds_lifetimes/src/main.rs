use std::fmt::Display;

/*
    This function uses everything covered in chapter 10 of the rust language book.
    A lifetime annotation is used to ensure that the return value of the function has the same lifetime as the two string slice parameters.
    A generic type parameter is used to allow the function to accept arguments of different types.
    A trait bound is used to limit the generic type parameter to types that implement the Display trait.
    The Display trait is used in the body of the function, to print the value of the ann parameter to the standard output.
    After that, the function returns the longer of the two string slices.
*/

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Hello world!");
    println!("The longest string is {}", result);
}
