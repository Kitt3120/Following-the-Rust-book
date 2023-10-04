const LIMIT: i32 = 15;

fn main() {
    let mut counter: i32 = 0;
    loop {
        println!("Counter now at {}", counter);
        counter += 1;

        if counter > LIMIT {
            break;
        }
    }

    println!("Goodbye!");
}
