pub fn bad() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let numbers = vec![10, 20, 50];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    //Duplicated code that can be extracted into a function
}

//====================================================================================================

fn largest_number(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn good() {
    let numbers = vec![1, 2, 3, 4, 5];
    let largest = largest_number(&numbers);
    println!("The largest number is {}", largest);

    let numbers = vec![10, 20, 50];
    let largest = largest_number(&numbers);
    println!("The largest number is {}", largest);

    //The duplicated code is extracted into a function
}
