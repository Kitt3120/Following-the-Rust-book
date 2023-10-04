struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn has_area(&self) -> bool {
        self.area() > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle::create(10, 20);

    if rect.has_area() {
        println!("Rectangle has an area of {} square pixels.", rect.area());
    } else {
        println!("Rectangle has no area.");
    }

    if rect2.has_area() {
        println!("Rectangle 2 has an area of {} square pixels.", rect2.area());
    } else {
        println!("Rectangle 2 has no area.");
    }

    if rect.has_area() && rect2.has_area() {
        if rect.can_hold(&rect2) {
            println!("Rectangle 1 can hold Rectangle 2.");
        } else {
            println!("Rectangle 1 cannot hold Rectangle 2.");
        }
    }

    if rect3.has_area() {
        println!(
            "Rectangle 3 is also cool. It has an area of {} square pixels.",
            rect3.area()
        );
    } else {
        println!("Rectangle 3 is also cool, but it has no area.");
    }
}
