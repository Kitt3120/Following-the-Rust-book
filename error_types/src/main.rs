use std::io::{self, Write};

use rand::Rng;

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Option<Guess> {
        if value < 1 || value > 100 {
            return None;
        }
        Some(Guess { value })
    }
}

fn main() {
    let number = rand::thread_rng().gen_range(0..=100);
    let mut tries = 0;

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Flushing stdout");

        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer);
        if result.is_err() {
            println!("Failed to read line: {}", result.err().unwrap());
            continue;
        }

        let input = buffer.trim().parse::<i32>();
        if input.is_err() {
            println!("Could not parse input to i32: {}", input.err().unwrap());
            continue;
        }
        let input = input.unwrap();

        let guess = Guess::new(input);
        if guess.is_none() {
            println!("Your guess must be between 1 and 100.");
            continue;
        }
        let guess = guess.unwrap().value;
        tries += 1;

        if guess == number {
            println!("You win! It took you {} tries.", tries);
            break;
        } else if guess < number {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }
}
