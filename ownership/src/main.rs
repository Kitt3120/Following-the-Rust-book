fn main() {
    ownership_moving();
    ownership_functions();
    ownership_functions_returns();
}

//EXAMPLE: Ownership of variables
fn ownership_moving() {
    //Allocates memory for "Hi" on heap, then stores the pointer to that on the stack in "string"
    let string: String = String::from("Hi!");
    //"string2" now also stores a pointer to "Hi" on the heap and "string" gets invalidated.
    let string2 = string; //This is called a move and is basically a shallow copy + invalidating the source.

    //Will throw an error because "string" is already moved.
    //println!("String: {}", string);

    //This works though.
    println!("String2: {}", string2);

    //However, if a deep copy is needed, clone() can help. This will also NOT invalidate the source.
    let string3 = string2.clone(); //This will copy the heap data and store a different pointer, pointing to the new heap data, in "string3".

    println!("String 2 again: {}\nString 3: {}", string2, string3); //Thus string2 and string3 are valid.
}

//EXAMPLE: Ownership of variables when using functions
fn ownership_functions() {
    let string: String = String::from("Hello there!");

    // string's value moves into the function...
    takes_ownership(string);
    // ... and so is no longer valid here

    // x comes into scope
    let x = 5;

    // x would move into the function,
    makes_copy(x);
    // but i32 has the Copy trait, so Copy-by-Value is applied and thus it's okay to still use x afterward
} // Here, x goes out of scope, then string. But because string's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

//EXAMPLE: Ownership of variables when using functions with return types
fn ownership_functions_returns() {
    let some_string: String = gives_ownership();
    println!("Some string: {}", some_string);

    let same_generated_string_on_heap: String = takes_and_gives_ownership(some_string);
    //some_string is now invalidated because the ownership has been taken by takes_and_gives_ownership.
    println!("Same on heap: {}", same_generated_string_on_heap); //it has been transferred to same_generated_string_on_heap.
}

fn gives_ownership() -> String {
    return String::from("Hi!");
}

fn takes_and_gives_ownership(some_string: String) -> String {
    return some_string;
}
