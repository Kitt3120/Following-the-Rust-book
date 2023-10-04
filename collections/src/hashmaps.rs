use std::collections::HashMap;

pub fn introduction() {
    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("Red"), 250);
    hashmap.insert(String::from("Blue"), 500);

    println!("Scoreboard:");
    for (key, value) in &hashmap {
        //We use a reference to the hash map here because we don't want to give into_iter() ownership of the hash map.
        println!("Team {}: {} Points", key, value);
    }

    let score_blue = hashmap.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("Blue team score: {}", score_blue);

    let score_yellow = hashmap.get(&String::from("Yellow")).copied().unwrap_or(0);
    println!("Yellow team score: {}", score_yellow);

    /*
    For types that implement the Copy trait, like i32, the values are copied into the hash map.
    For owned values like String, the values will be moved and the hash map will be the owner of those values.
     */
    let mut hashmap = HashMap::new();
    let key = String::from("Hi");
    let value = 5;
    hashmap.insert(key, value);

    //Notice that we can still access value,
    //but we can't access key anymore because it was moved into the hash map.
    //println!("key: {}", key);
    println!("value: {}", value);

    //We can use references to the key and value instead of moving them into the hash map.
    let mut hashmap = HashMap::new();
    let key = String::from("Red");
    let value = 5;
    hashmap.insert(&key, &value);

    println!("key: {}", key);
    println!("value: {}", value);

    //However, we must dereference the values when we retrieve them from the hash map.
    println!("Scoreboard:");
    for (key, value) in &hashmap {
        println!("Team {}: {} Points", key, value); //println! does this for us automatically
    }

    //Updating/Overwriting values
    let key = String::from("Red");
    let value = 250;
    hashmap.insert(&key, &value);
    println!("Scoreboard:");
    for (key, value) in &hashmap {
        println!("Team {}: {} Points", &key, &value); //println! does this for us automatically
    }

    //Only insert if not exists
    hashmap.entry(&key).or_insert(&500);
    println!("Scoreboard:");
    for (key, value) in &hashmap {
        println!("Team {}: {} Points", &key, &value); //println! does this for us automatically
    }
    //Notice that the value for Red is still 250 because we only inserted 500 if the key didn't exist.
}
