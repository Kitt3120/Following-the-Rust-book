use std::{env, path::Path};

use grep::parse_arguments;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let args = parse_arguments(&args);

    if args.is_err() {
        println!("Error while parsing arguments: {}", args.unwrap_err());
        return;
    }
    let args = args.unwrap();

    let content = grep::read_file(&Path::new(&args.path));
    if content.is_err() {
        println!("Error while reading file: {}", content.unwrap_err());
        return;
    }
    let content = content.unwrap();

    let content = grep::filter(&content, &args.pattern);

    println!("{}", content.join("\n"));
}
