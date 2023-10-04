fn main() {
    find_words_without_slices();
    find_words_with_slices();
}

//EXAMPLE: Handling strings without slices
fn find_words_without_slices() {
    let mut string: String = String::from("This is a test");
    let index_first_word_end = get_first_word_without_slices(&string);

    print!("String: {}\nFirst word is: ", string);

    for index in 0..index_first_word_end {
        print!("{}", string.chars().nth(index).unwrap());
    }
    print!("\n");

    //Now if string changes, the index will not be updated
    string = String::from("");
    print!("String: {}\nFirst word is: ", string); //Still says that the first word ends at index 4

    for index in 0..index_first_word_end {
        let char: Option<char> = string.chars().nth(index);
        match char {
            Some(_) => print!("{}", string.chars().nth(index).unwrap_or('_')),
            None => println!("Char at {} was None", index),
        }
    }
    print!("\n");

    //Also, to get another word, we would need another function that returns a tuple
    string = String::from("Word1 Word2 Word3");
    let (index1, index2) = get_second_word_without_slices(&string);
    println!("String: {}\nSecond word is: ", string,);
    for index in index1..index2 {
        let char: Option<char> = string.chars().nth(index);
        match char {
            Some(_) => print!("{}", string.chars().nth(index).unwrap_or('_')),
            None => println!("Char at {} was None", index),
        }
    }
    print!("\n");
}

fn get_first_word_without_slices(string: &String) -> usize {
    let string_as_bytes = string.as_bytes();

    for (i, &item) in string_as_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return string_as_bytes.len();
}

fn get_second_word_without_slices(string: &String) -> (usize, usize) {
    let mut index1: usize = 0;

    for (i, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            if index1 == 0 {
                index1 = i;
            } else {
                return (index1, i);
            }
        }
    }

    return (index1, string.len());
}

//EXAMPLE: Handling string with slices
fn find_words_with_slices() {
    let mut string: String = String::from("This is a test");
    let first_word: &str = get_first_word_with_slices(&string);

    println!("String: {}\nFirst word is: {}", string, first_word);

    //Now if string changes, the slice reference will be invalid
    string = String::from("");
    //println!("String: {}\nFirst word is {}", string, first_word);

    //Also, to get another word, we still need another function, but the return type is the same
    string = String::from("Word1 Word2 Word3");
    let second_word: &str = get_second_word_with_slices(&string);
    println!("String: {}\nSecond word is {}", string, second_word);
}

fn get_first_word_with_slices(string: &String) -> &str {
    for (i, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    return string;
}

fn get_second_word_with_slices(string: &String) -> &str {
    let mut index1: usize = 0;

    for (i, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            if index1 == 0 {
                index1 = i;
            } else {
                return &string[index1..i];
            }
        }
    }

    return &string[index1..string.len()];
}
