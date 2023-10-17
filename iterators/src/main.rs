/*
    Looping over elements with a for loop usually consists of 4 steps in other programming languages:
    1. Defining a counter variable
    2. Checking if the counter variable has reached the end
    3. Performing an action on the current element
    4. Increasing the counter variable

    In Rust, this logic has been abstracted away into the Iterator trait.
    Iterators have a method called next() that returns an Option<T> (Some(T) or None) to iterate over all elements.
    The for loop will stop when the iterator returns None.
    The for loop will automatically call next() on the iterator.
    The for loop also consumes the iterator, so it cannot be used again after the loop.

    Iterators are lazy, which means that they will not do anything unless they are consumed.
    This means that you can chain multiple iterators together without any performance penalty.
    In fact, using iterators is often more performant than using manual loops because the compiler can optimize them better.
    For example, the compiler can unroll the loop if it knows how many iterations there will be.
    This removes the overhead of checking if the counter variable has reached the end at runtime.
*/

use std::vec;

fn main() {
    println!("===== Simple Iterators =====");
    simple_iterators();

    println!("\n===== Consuming Adaptors =====");
    consuming_adaptors();

    println!("\n===== Iterator Adaptors =====");
    iterator_adaptors();

    println!("\n===== Closure with Iterator =====");
    closure_with_iterator();
}

fn simple_iterators() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();

    println!("First loop:");
    for number in numbers_iter {
        println!("{}", number);
    }

    // or

    println!("Second loop:");
    for number in numbers.iter() {
        println!("{}", number);
    }

    // or even better

    println!("Third loop:");
    for number in &numbers {
        println!("{}", number);
    }
    // Note that we are using a reference to the vector here to avoid moving ownership into the for loop.

    // If we instead do this:
    println!("Fourth loop:");
    for number in numbers {
        println!("{}", number);
    }
    // numbers is moved and cannot be used anymore.

    // This would not compile:
    //print!("{:?}", numbers);
}

/*
    As stated before, iterators can be chained together.
    By doing this, you will chain together two types of iterators.
    The first type is called consuming adaptors.
    For example, the sum() method is a consuming adaptor.
*/
fn consuming_adaptors() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    let sum = numbers_iter.sum::<i32>(); // Turbofish syntax or type annotation is needed here
    println!("Sum: {}", sum);
    // Here, we are using the sum() method on the iterator, which will consume it.
}

/*
    On the other hand, there are also non-consuming adaptors, called iterator adaptors.
    These adaptors will return a new iterator, which can be used again.
    For example, the map() method will return a new iterator that will apply a function to each element.
    You can chain multiple iterator adaptors together and then use a consuming adaptor on them.
    In other programming languages, this would most likely lead to multiple loops and cloning of data.
    In Rust, this is not the case, as iterators are lazy and will only do something when they are consumed.
*/
fn iterator_adaptors() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    let numbers_doubled_iter = numbers_iter.map(|x| x * 2);
    let sum = numbers_doubled_iter.sum::<i32>();
    println!("Sum: {}", sum);

    // or
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().map(|x| x * 2).sum::<i32>();
    println!("Sum: {}", sum);
}

// Iterators go well with closures:
fn closure_with_iterator() {
    let numbers = vec![1, 2, 3, 4, 5];

    let add_together_some = |numbers: Vec<i32>| -> i32 { numbers.into_iter().sum() };
    /*
       Note that we are using the into_iter() method here.
       The normal iter() method will return an iterator with just references to the elements.
       We want to work with actual values, so we move the ownership of the vector into the iterator by using into_iter().
       There is also an iter_mut() method, which is like iter() but for mutable references.
    */

    let sum = add_together_some(numbers); // This is horrible naming btw, don't do this in production code :p
    println!("Sum: {}", sum);

    // Or a better example:

    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_above_3 = get_numbers_above_size(numbers, 3);
    println!("Numbers above 3: {:?}", numbers_above_3);
}

fn get_numbers_above_size(numbers: Vec<i32>, size: i32) -> Vec<i32> {
    numbers.into_iter().filter(|x| x > &size).collect()
}
