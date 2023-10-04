//Bad:

fn largest_number_bad(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char_bad(chars: &[char]) -> &char {
    let mut largest = &chars[0];
    for char in chars {
        if char > largest {
            largest = char;
        }
    }
    largest
}

pub fn bad() {
    let numbers = vec![1, 2, 3, 4, 5];
    let largest = largest_number_bad(&numbers);
    println!("The largest number is {}", largest);

    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    let largest = largest_char_bad(&chars);
    println!("The largest character is {}", largest);

    //Two functions are used for the same purpose, only for a different type.
}

//The two functions are identical except for the type of the parameter and the type of the return value.

//====================================================================================================

//Good:

/*
    Setting the PartialOrd trait bound on the generic type T means the largest_element function will only work on types that implement the PartialOrd trait.
    Imagine it like a contract: We open up our function to be used with any type (generic), but in return, the type must implement the PartialOrd trait.
*/
fn largest_element<T: PartialOrd>(elements: &[T]) -> &T {
    let mut largest = &elements[0];
    for element in elements {
        if element > largest {
            largest = element;
        }
    }
    largest
}

pub fn good() {
    let numbers = vec![1, 2, 3, 4, 5];
    let largest = largest_element(&numbers);
    println!("The largest number is {}", largest);

    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    let largest = largest_element(&chars);
    println!("The largest character is {}", largest);

    //The two functions are replaced by a single function that uses generics.
}
