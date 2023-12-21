/*
    In Rust, global variables are called static variables.
    Static variables are similar to constants.
    The names of static variables are in SCREAMING_SNAKE_CASE by convention.

    Static variables can only store references with the 'static lifetime,
    which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.

    Accessing an immutable static variable is safe, just like a constant.
    But accessing a mutable static variable is unsafe.

    The difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
    Constants, on the other hand, are allowed to duplicate their data whenever they are used.

    I wanted to give an example of this below, but I couldn't come up with a way to trigger the effect.
    Whatever I tried, the const always had the same address when used, just as the immutable static variable.

    GitHub Copilot states the following:

    "The Rust documentation states that constants are inlined at their usage points and have no fixed address in memory.
    However, in practice, you might not observe this behavior due to optimizations performed by the Rust compiler or the LLVM backend.

    In the case of your code,
    it seems like the compiler has decided to optimize the constant usage by keeping it at a fixed memory address,
    similar to a static variable.
    This is likely because the constant is being used in a context
    where the compiler can determine that inlining the constant and duplicating its data would not provide any benefits.

    If you want to observe the difference between constants and static variables,
    you might need a more complex example where the compiler can't easily optimize the constant usage.
    However, keep in mind that the exact behavior can depend on many factors,
    including the specific version of the Rust compiler and its optimization settings."
*/

const CONST: &str = "I am a constant!";
static I_S_VARIABLE: &str = "Rust is cool!";
static mut M_S_VARIABLE: &str = "Rust is bad!";

pub fn run() {
    print_strings();

    println!();
    println!("Applying fix");
    unsafe {
        fix_string();
    }

    println!();
    print_strings();
}

fn print_strings() {
    println!("CONST: {} (Address: {:p})", CONST, &CONST as *const &str);

    println!(
        "COOL_STR: {} (Address: {:p})",
        I_S_VARIABLE, &I_S_VARIABLE as *const &str
    );

    unsafe {
        println!(
            "DYNAMIC_STR: {} (Address: {:p})",
            M_S_VARIABLE, &M_S_VARIABLE as *const &str
        );
    }
}

unsafe fn fix_string() {
    M_S_VARIABLE = "Rust is good!";
}
