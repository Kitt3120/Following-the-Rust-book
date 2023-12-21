pub fn run() {
    println!("======= The let statement =====");
    let_statement();

    println!();

    println!("===== The if let statement =====");
    if_let_statement();

    println!();

    println!("===== The while let statement =====");
    while_let_statement();

    println!();

    println!("===== The match statement =====");
    match_statement();

    println!();

    println!("===== The for statement =====");
    for_statement();

    println!();

    println!("===== Function parameters =====");
    function_parameters();
}

// let PATTERN = EXPRESSION;
fn let_statement() {
    let _x = 5;
    let (_x, _y) = (1, 2);
    let (x, y, ..) = (1, 2, 3, 4, 5);
    println!("x: {}, y: {}", x, y);
}

// if let PATTERN = EXPRESSION { EXPRESSION }
fn if_let_statement() {
    let x = Some(5);
    if let Some(y) = x {
        println!("y: {}", y);
    }
}

// while let PATTERN = EXPRESSION { EXPRESSION }
fn while_let_statement() {
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("x: {}", x);
    }
}

/*
    match EXPRESSION {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
*/
fn match_statement() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
}

// for PATTERN in EXPRESSION { EXPRESSION }
fn for_statement() {
    let v = vec![1, 2, 3, 4, 5];
    for x in v {
        println!("x: {}", x);
    }
}

// function parameters are also patterns
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

fn function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}
