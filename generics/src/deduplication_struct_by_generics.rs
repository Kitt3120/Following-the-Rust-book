//Bad:

#[derive(Debug)]
struct PointWithIntegers {
    _x: i32,
    _y: i32,
}

#[derive(Debug)]
struct PointWithFloats {
    _x: f32,
    _y: f32,
}

pub fn bad() {
    let p1 = PointWithIntegers { _x: 5, _y: 10 };
    let p2 = PointWithFloats { _x: 1.0, _y: 4.0 };
    let p3 = PointWithIntegers { _x: 5, _y: 10 };
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p3: {:?}", p3);
}

//====================================================================================================

//Good:

#[derive(Debug)]
struct Point<T> {
    _x: T,
    _y: T,
}

#[derive(Debug)]
struct CoolerPoint<T, U> {
    _x: T,
    _y: U,
}

pub fn good() {
    let p1 = Point { _x: 5, _y: 10 };
    let p2 = Point { _x: 1.0, _y: 4.0 };
    let p3 = Point { _x: 5, _y: 10 };
    //let _wont_work = Point { x: 5, y: 4.0 };
    let will_work = CoolerPoint { _x: 5, _y: 4.0 };

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p3: {:?}", p3);
    println!("will_work: {:?}", will_work);
}
