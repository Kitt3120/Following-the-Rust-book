/*
    See associated_types.rs and default_generic_types.rs first.

    One place where the Rust standard library makes use of default generic types is the Add trait.
    When implementing the Add trait on a type, it overloads the + operator for that type.

    The Add trait allows you to define what the right-hand side of the + operator should be for your type.
    But if you don't give it a type, it defaults to Self, which means that it'll be the same type as the type you're implementing it on.
*/

use std::{fmt::Display, ops::Add};

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// No generic type here, so it defaults to Self, which is Point.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Here, we're adding the ability to add an i32 to a Point. The "other" parameter is now an i32. It still returns a Point.
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// Because we don't always want to move the values into the function, we define an Add trait that takes a reference instead.
impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// As this still moves the instance it is called on, we can also implement Add for a reference to the type.
impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn run() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    println!("p1 = {}", p1);
    println!("p2 = {}", p2);

    // This works because we implemented Add for Point with the default generic type.
    let p3 = p1 + p2;
    println!("p1 + p2 = {}", p3);

    /*
        But notice that p1 and p2 are moved now.
        We can't use them anymore.
        That's why we implemented Add for Point with a reference!
    */
    println!();

    let longer_living_p1 = Point { x: 1, y: 2 };
    let longer_living_p2 = Point { x: 3, y: 4 };
    println!("longer_living_p1 = {}", longer_living_p1);
    println!("longer_living_p2 = {}", longer_living_p2);

    let long_living_p3 = longer_living_p1 + &longer_living_p2;
    println!("longer_living_p1 + &longer_living_p2 = {}", long_living_p3);

    // Now we could still use long_living_p2.
    println!("longer_living_p2 = {}", longer_living_p2);

    // However, we can't use long_living_p1 anymore, it still got moved.

    println!();
    let longest_living_p1 = &Point { x: 1, y: 2 };
    let longest_living_p2 = &Point { x: 3, y: 4 };
    println!("longest_living_p1 = {}", longest_living_p1);
    println!("longest_living_p2 = {}", longest_living_p2);

    let longest_living_p3 = longest_living_p1 + longest_living_p2;
    println!(
        "longest_living_p1 + longest_living_p2 = {}",
        longest_living_p3
    );

    println!("And they can still be used:");
    println!("longest_living_p1 = {}", longest_living_p1);
    println!("longest_living_p2 = {}", longest_living_p2);
    println!("longest_living_p3 = {}", longest_living_p3);
}
