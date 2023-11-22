/*
    The example in for_polymorphism.rs works but could still be confusing, hence the better example below taken from the Rust book.
*/

trait Draw {
    fn draw(&self);
}

struct Screen {
    title: String,
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new(title: String, components: Vec<Box<dyn Draw>>) -> Screen {
        Screen { title, components }
    }
}

// Little deviation from the book here, why not implement Draw for Screen? This way, you could have screens within screens.
impl Draw for Screen {
    fn draw(&self) {
        println!("Drawing screen {}", self.title);

        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    label: String,
    width: u32,
    height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing button {} with width {} and height {}",
            self.label, self.width, self.height
        );
    }
}

struct SelectBox {
    options: Vec<String>,
    width: u32,
    height: u32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing select box with width {}, height {} and options {:?}",
            self.width, self.height, self.options
        );
    }
}

pub fn run() {
    let inner_screen = Screen::new(
        String::from("Inner Screen"),
        vec![
            Box::new(Button {
                label: String::from("Cancel"),
                width: 50,
                height: 10,
            }),
            Box::new(SelectBox {
                options: vec![
                    String::from("Option 1"),
                    String::from("Option 2"),
                    String::from("Option 3"),
                ],
                width: 50,
                height: 10,
            }),
        ],
    );

    let outer_screen = Screen::new(
        String::from("Outer Screen"),
        vec![
            Box::new(Button {
                label: String::from("Exit"),
                width: 50,
                height: 10,
            }),
            Box::new(inner_screen),
        ],
    );

    outer_screen.draw();
}

/*
    Other developers using our crate will now be able to implement the Draw trait on types of their own and use them within a Screen instance
    thanks to dynamic dispatch.
*/
