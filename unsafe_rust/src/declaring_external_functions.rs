/*
    As shown in the example of external_functions.rs, you can call functions from other languages in Rust.
    But Rust also allows you to define the ABI for your own binaries.
    This is useful if you want to call Rust functions from other languages.

    What's important is to add the #[no_mangle] attribute to the function.
    This tells the Rust compiler not to mangle the name of the function.
    Mangling is a technique used by compilers to change the name of a function to optimize it for other parts of the compilation process.

    In comparison to using external functions, you don't need to mark the function as unsafe,
    as Rust guarantees that the function is safe to call from other languages.
*/

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

#[no_mangle]
pub extern "C" fn abs(input: i32) -> i32 {
    if input < 0 {
        -input
    } else {
        input
    }
}
