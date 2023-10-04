//Bad:

struct PointWithIntegers {
    _x: i32,
    _y: i32,
}

struct PointWithFloats {
    _x: f32,
    _y: f32,
}

pub fn bad() {
    let _p1 = PointWithIntegers { _x: 5, _y: 10 };
    let _p2 = PointWithFloats { _x: 1.0, _y: 4.0 };
    let _p3 = PointWithIntegers { _x: 5, _y: 10 };
}

//====================================================================================================

//Good:

struct Point<T> {
    _x: T,
    _y: T,
}

struct CoolerPoint<T, U> {
    _x: T,
    _y: U,
}

pub fn good() {
    let _p1 = Point { _x: 5, _y: 10 };
    let _p2 = Point { _x: 1.0, _y: 4.0 };
    let _p3 = Point { _x: 5, _y: 10 };
    //let _wont_work = Point { x: 5, y: 4.0 };
    let _will_work = CoolerPoint { _x: 5, _y: 4.0 };
}
