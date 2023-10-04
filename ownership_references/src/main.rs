fn main() {
    println!("= Ownership =");
    ownership();
    println!("= Ownership with references/borrowing =");
    ownership_with_references();
}

//EXAMPLE: Ownership and its problem
fn ownership() {
    //ownership now owns string
    let string: String = String::from("Hi!");

    //ownership of string is taken by take_ownership
    take_ownership(string);
    //string can't be used at this point

    //Using string at this point would throw an error
    println!("string can't be used anymore!");

    //To prevent this with the methods learned so far, we would always have to return the ownership back to the caller scope

    //ownership now owns string2
    let mut string2: String = String::from("Hi again!");

    //ownership of string2 is taken by take_and_give_back_ownership and given back afterwards
    string2 = take_and_give_back_ownership(string2);

    //string2 can still be used
    println!("string2 is: {}", string2)

    //However, this is a lot of boilterplate/overhead to have this basic functionality that other programming langugages have by default.
    //It seems more like a work-around than a solution.
    //This is why Rust came up with a solution to this: Using references. This is also called borrowing.
}

fn take_ownership(_string: String) {}

fn take_and_give_back_ownership(string: String) -> String {
    return string;
}

//EXAMPLE: Solution to the problem
fn ownership_with_references() {
    //ownership_with_references now owns string
    let string: String = String::from("Hi!");

    //borrow_string is called and it will temporarily borrow string while being in its scope
    borrow_string(&string);

    //string can still be used as the ownership has not changed
    println!("string is: {}", string);

    //ownership_with_references now owns another_string
    let mut another_string: String = String::from("Hello there!");
    //borrowed variables can also be modified if borrowed as mutable reference. However, the source must be mutable, of course!
    borrow_string_for_modification(&mut another_string);

    //another_string can still be used as the ownership has not changed
    println!("another_string is: {}", another_string);
}

fn borrow_string(_string: &String) {}

fn borrow_string_for_modification(string: &mut String) {
    string.push_str(" How are you?");
}
