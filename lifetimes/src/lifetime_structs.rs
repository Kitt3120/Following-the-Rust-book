struct Novel<'a> {
    name: &'a String,
}

pub fn introduce() {
    let name = &String::from("The Hobbit");
    let novel = Novel { name };
    println!("The novel is called {}", novel.name);
}
