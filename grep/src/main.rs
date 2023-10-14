use grep::{parse_arguments, run};
use std::{
    env::{self},
    process::exit,
};

fn main() {
    let arguments = env::args().collect::<Vec<String>>();

    let arguments = parse_arguments(&arguments).unwrap_or_else(|error| {
        eprintln!("Error while parsing arguments: {error}");
        exit(1);
    });

    run(arguments).unwrap_or_else(|error| {
        eprintln!("Error while running: {error}");
        exit(1);
    });
}
