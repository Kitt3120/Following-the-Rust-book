use core::fmt;
use std::fmt::Display;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }
}

struct Human {
    _name: String,
    _age: u32,
    _height: u32,
    _weight: u32,
    location: Location,
}

#[allow(dead_code)]
impl Human {
    fn new(name: String, age: u32, height: u32, weight: u32, x: i32, y: i32) -> Human {
        Human {
            _name: name,
            _age: age,
            _height: height,
            _weight: weight,
            location: Location::new(x, y),
        }
    }

    fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.location.y += 1,
            Direction::Down => self.location.y -= 1,
            Direction::Left => self.location.x -= 1,
            Direction::Right => self.location.x += 1,
        }
    }
}

struct Car {
    _make: String,
    _model: String,
    _year: u32,
    engine_started: bool,
    location: Location,
}

#[allow(dead_code)]
impl Car {
    fn new(make: String, model: String, year: u32, x: i32, y: i32) -> Car {
        Car {
            _make: make,
            _model: model,
            _year: year,
            engine_started: false,
            location: Location::new(x, y),
        }
    }

    fn start_engine(&mut self) {
        self.engine_started = true;
    }

    fn stop_engine(&mut self) {
        self.engine_started = false;
    }

    fn drive(&mut self, direction: Direction) {
        if self.engine_started {
            match direction {
                Direction::Up => self.location.y += 1,
                Direction::Down => self.location.y -= 1,
                Direction::Left => self.location.x -= 1,
                Direction::Right => self.location.x += 1,
            }
        }
    }
}

// Human and Car have things common:
// - They both have a location
// - They both can move

// We can use a trait to define the common things
trait Movable {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn get_location(&self) -> &Location;
    fn move_to(&mut self, x: i32, y: i32);
}

// And implement the trait for both Human and Car
impl Movable for Human {
    fn get_location(&self) -> &Location {
        &self.location
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let x_diff = x - self.location.x;
        let y_diff = y - self.location.y;

        if x_diff > 0 {
            for _ in 0..x_diff {
                self.walk(Direction::Right);
            }
        } else if x_diff < 0 {
            for _ in 0..x_diff.abs() {
                self.walk(Direction::Left);
            }
        }

        if y_diff > 0 {
            for _ in 0..y_diff {
                self.walk(Direction::Up);
            }
        } else if y_diff < 0 {
            for _ in 0..y_diff.abs() {
                self.walk(Direction::Down);
            }
        }
    }
}

impl Movable for Car {
    fn get_location(&self) -> &Location {
        &self.location
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let engine_was_started = self.engine_started;
        if !engine_was_started {
            self.start_engine();
        }

        let x_diff = x - self.location.x;
        let y_diff = y - self.location.y;

        if x_diff > 0 {
            for _ in 0..x_diff {
                self.drive(Direction::Right);
            }
        } else if x_diff < 0 {
            for _ in 0..x_diff.abs() {
                self.drive(Direction::Left);
            }
        }

        if y_diff > 0 {
            for _ in 0..y_diff {
                self.drive(Direction::Up);
            }
        } else if y_diff < 0 {
            for _ in 0..y_diff.abs() {
                self.drive(Direction::Down);
            }
        }

        if !engine_was_started {
            self.stop_engine();
        }
    }
}

pub fn run() {
    //Now you can choose:
    let mut movable = Human::new(String::from("John"), 32, 180, 80, 0, 0);
    //let mut movable = Car::new(String::from("Ford"), String::from("Focus"), 2015, 0, 0);

    print_movable(&movable);

    println!("Moving...");
    movable.move_to(50, 25);

    print_movable(&movable);
}

// We can also use a trait as a parameter restriction
fn print_movable(movable: &impl Movable) {
    println!(
        "{} is at ({}, {})",
        movable.type_name(),
        movable.get_location().x,
        movable.get_location().y
    );
}

// Which is syntax sugar for
//fn print_movable<T: Movable>(movable: &T)

// You can combine multiple traits with +
//fn do_something_with_displayable_movables(movable: &(impl Movable + Display)) {

// Or use it as a return type, which is especially useful for factory patterns
//fn create_movable() -> impl Movable

// You can also define conditional trait implementations
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
            let s = 3.to_string();
        }
    }
}

// Or you can define functions for any type that implements a trait
// This is called a blanket implementation
//impl<T: Display> ToString for T
