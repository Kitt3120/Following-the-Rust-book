mod hashmaps;
mod strings;
mod vectors;

fn main() {
    print_title("Vectors");
    vectors::introduction();
    println!();
    print_title("Strings");
    strings::introduction();
    println!();
    print_title("Hashmaps");
    hashmaps::introduction();
}

fn print_title(title: &str) {
    let len = title.chars().count() + 8;
    let bar = "=".repeat(len);
    println!("{}", bar);
    println!("=== {} ===", title);
    println!("{}", bar);
}

/*
Example exercises:

Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
and mode (the value that occurs most often; a hash map will be helpful here) of the list.

Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!

Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
 */
