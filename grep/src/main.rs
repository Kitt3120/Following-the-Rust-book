use grep::{filter, parse_arguments, read_file};
use std::{env, path::Path, process::exit};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let args = parse_arguments(&args).unwrap_or_else(|error| {
        println!("Error while parsing arguments: {error}");
        exit(1);
    });

    let content = read_file(&Path::new(&args.path));
    let content = content.unwrap_or_else(|error| {
        println!("Error while reading file: {error}");
        exit(1);
    });

    let lines = filter(&content, &args.pattern);
    println!("{}", lines.join("\n"));
}
