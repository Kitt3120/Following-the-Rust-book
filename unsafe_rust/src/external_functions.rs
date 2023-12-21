/*
    You may have heard of a thing called "bindings".
    An example is the Numpy library for Python, which uses bindings to call C code from Python.
    This makes the Numpy library extremely fast, as the heavy lifting is done in C.

    In the background, this works because applications have a so called Application Binary Interface (ABI).
    This enables binaries to call functions from each other, even if they are written in different languages.

    Rust can also call functions from other languages, as long as they have a C ABI.
    But because Rust can't apply its safety checks to these functions, you have to mark them as unsafe.
*/

pub fn run() {
    let input = -3;

    let output = unsafe { abs(input) };

    println!("Absolute of {} according to C: {}", input, output);
}

extern "C" {
    fn abs(input: i32) -> i32;
}
