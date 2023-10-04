fn main() {
    println!("Hello, world!");
    another_function();
    say("Haya!");

    println!("5 + 4 is {}", add(5, 4));
    println!("1 + 2 + 3 + 4 + 5 is {}", add_array(&[1, 2, 3, 4, 5]));

    let mut cool_array = [1, 2, 3];
    println!("Cool array: {}", add_array(&cool_array));
    multiply_array(&mut cool_array, 2);
    println!("Cool array * 2: {}", add_array(&cool_array));

    let mut number = 5;
    println!("Before overwriting: {number}");
    overwrite(&mut number, 3);
    println!("After overwriting: {number}");
}

fn another_function() {
    println!("Hi!");
}

fn say(text: &str) {
    println!("{text}");
}

fn add(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

fn add_array(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    sum
}

fn multiply_array(numbers: &mut [i32], factor: i32) {
    for index in 0..numbers.len() {
        numbers[index] = numbers[index] * factor;
    }
}

fn overwrite(number: &mut i32, value: i32) {
    *number = value;
}
