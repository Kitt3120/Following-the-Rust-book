//Bad example:

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
}

//The two functions are identical except for the type of the parameter and the type of the return value.

//====================================================================================================

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

    //The duplicated code is extracted into a function
}
