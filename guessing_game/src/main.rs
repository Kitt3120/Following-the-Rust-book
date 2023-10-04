use rand::Rng;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

fn main() {
    let random_number = rand::thread_rng().gen_range(0..=10);

    loop {
        print!("Type in a number: ");
        stdout().flush().expect("Flushing stdout");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Reading number from stdin.");
        input = input.trim().to_string();

        if input.len() == 0 {
            continue;
        }

        let input: u32 = match input.parse() {
            //old input string is shadowed by new input u32
            Ok(number) => number,
            Err(_) => continue,
        };

        match input.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
