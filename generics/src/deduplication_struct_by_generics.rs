//Bad:

struct PointWithIntegers {
    x: i32,
    y: i32,
}

struct PointWithFloats {
    x: f32,
    y: f32,
}

pub fn bad() {
    let _p1 = PointWithIntegers { x: 5, y: 10 };
    let _p2 = PointWithFloats { x: 1.0, y: 4.0 };
    let _p3 = PointWithIntegers { x: 5, y: 10 };
}

//====================================================================================================

//Good:

struct Point<T> {
    x: T,
    y: T,
}

struct CoolerPoint<T, U> {
    x: T,
    y: U,
}

pub fn good() {
    let _p1 = Point { x: 5, y: 10 };
    let _p2 = Point { x: 1.0, y: 4.0 };
    let _p3 = Point { x: 5, y: 10 };
    //let _wont_work = Point { x: 5, y: 4.0 };
    let _will_work = CoolerPoint { x: 5, y: 4.0 };
}
