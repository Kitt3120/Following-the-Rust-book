pub fn introduction() {
    //Creating a string on heap and referencing it
    let mut _s = String::new();

    //Creating a string from a string literal
    let data = "Option 1";
    let _s = data.to_string();
    let _s = "Option 2".to_string();
    let _s = String::from("Option 3");

    //Strings are UTF-8 encoded
    let mut poop = String::from("💩");
    println!("Poop: {}", poop);

    //Appending to a string
    poop.push_str("💩");
    println!("More poop: {}", poop);

    //Using the + operator to append
    let mut poop = String::from("💩");
    poop = poop + "💩";
    println!("Poop poop: {}", poop);

    //Using the format! macro to append
    let mut poop = String::from("💩");
    poop = format!("{}", poop.repeat(3));
    println!("Poop poop poop: {}", poop);

    //Cirillic characters
    let hello = String::from("Здравствуйте");
    println!(
        "{} is just {} chars but {} bytes",
        hello,
        hello.chars().count(),
        hello.bytes().len()
    );

    let namaste = String::from("नमस्ते");
    println!(
        "{} is just {} chars but {} bytes",
        namaste,
        namaste.chars().count(),
        namaste.bytes().len()
    );
}
