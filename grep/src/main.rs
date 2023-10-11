use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Please provide at least two arguments\ngrep <pattern> <filename>",);
        return;
    }

    let var1 = &args[1];
    let var2 = &args[2];

    println!("var1: {}", var1);
    println!("var2: {}", var2);
}
