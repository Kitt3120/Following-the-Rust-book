//Bad:

#[derive(Debug)]
enum HoldingInteger {
    Something(i32),
}

#[derive(Debug)]
enum HoldingChar {
    Something(char),
}

pub fn bad() {
    let holding_integer = HoldingInteger::Something(1);
    let holding_char = HoldingChar::Something('a');
    println!("holding_integer: {:?}", holding_integer);
    println!("holding_char: {:?}", holding_char);
}

//====================================================================================================

//Good:

#[derive(Debug)]
enum Holding<T> {
    Something(T),
}

pub fn good() {
    let holding_integer = Holding::Something(1);
    let holding_char = Holding::Something('a');
    println!("holding_integer: {:?}", holding_integer);
    println!("holding_char: {:?}", holding_char);
}
