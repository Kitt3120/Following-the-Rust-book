struct Novel<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> Novel<'a> {
    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    fn set_description(&mut self, description: &'a str) {
        self.description = description;
    }
}

pub fn introduce() {
    let name = &String::from("The Hobbit");
    let description = &String::from("A book about a hobbit");
    let mut novel = Novel { name, description };
    println!("The novel is called {}", novel.name);

    let new_name = &String::from("The Lord of the Rings");
    let new_description = &String::from("A book about a ring");
    novel.set_name(&new_name);
    novel.set_description(&new_description);
    println!("The novel is called {}", novel.name);

    // Here we ensure that the novel's name and description are valid for the same lifetime
    // Which enables us to call
    print_longest(&novel.name, &novel.description);
    // Which requires both parameters to be valid for the same lifetime

    // And which would not work with
    let name = String::from("Steins;Gate");
    let description = String::from("A book about time travel");
    let mut anti_chrono_novel = Novel {
        name: &name,
        description: &description,
    };

    {
        let name = String::from("Steins;Gate 0");
        anti_chrono_novel.set_name(&name);
    }

    // This would lead to line 43 throwing an error
    //println!("The novel is called {}", anti_chrono_novel.name);
}

fn print_longest<'a>(string1: &'a str, string2: &'a str) {
    if string1.len() > string2.len() {
        println!("The longest string is {}", string1);
    } else {
        println!("The longest string is {}", string2);
    }
}
