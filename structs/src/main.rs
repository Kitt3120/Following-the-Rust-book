struct Point {
    x: isize,
    y: isize,
}

struct Rectangle {
    location: Point,
    width: isize,
    height: isize,
}

struct TupleStruct(isize, isize);

struct AlwaysEqual;

fn main() {
    let nice_location = create_point(5, 10);

    let cool_rectangle = create_rectangle(nice_location, 50, 100);

    let another_rectangle = Rectangle {
        width: 100,
        ..cool_rectangle
    };

    let some_tuple_struct = TupleStruct(0, 0);

    let always_equal_struct = AlwaysEqual;
}

fn create_point(x: isize, y: isize) -> Point {
    return Point { x, y };
}

fn create_rectangle(location: Point, width: isize, height: isize) -> Rectangle {
    return Rectangle {
        location,
        width,
        height,
    };
}
