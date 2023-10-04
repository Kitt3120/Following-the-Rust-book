use std::{
    fs::{self, File},
    io::Result,
    path::Path,
};

fn main() {
    //In Rust, there are 2 types of errors: recoverable and unrecoverable.
    //Recoverable errors are ones that a program can reasonably be expected to recover from.
    //Developers are, by design of the Rust language, not allowed to ignore those errors. They must be handled in some way.
    //Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.
    //Because these are always bugs, Rust doesn’t try to enforce that you handle or recover from these errors in any particular way.
    //Instead, it terminates programs when unrecoverable errors occur, because it can’t safely proceed.

    let path = Path::new("./Cargo1.toml");
    //Example of a recoverable error:
    println!("The following is a recoverable error:");
    let file = function_that_may_fail(path);
    match file {
        Ok(_file) => println!("File opened successfully!"),
        Err(error) => println!("Error opening file: {:?}", error),
    }

    //Example for a recoverable error that has been propagated:
    println!("The following is a recoverable error that has been propagated:");
    let file_content = function_that_will_propagate(path);
    match file_content {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error opening file: {:?}", error),
    }

    //Example of an unrecoverable error:
    println!("The program will now panic because of an unrecoverable error:");
    function_that_will_panic();
}

fn function_that_may_fail(path: &Path) -> Result<File> {
    File::open(path)
}

fn function_that_will_propagate(path: &Path) -> Result<String> {
    let file = function_that_may_fail(path);
    if let Err(error) = file {
        return Err(error);
    }

    let content = fs::read_to_string(path);
    content
}

fn function_that_will_panic() {
    panic!("Error!");
}
