/*
    The newtype pattern gives compile time guarantees that the right type of value is supplied to a program.
    This can be used to avoid mixing up different values that are stored as the same primitive type.
    By doing this, the public API of a type can be made more clear and less error prone.
    As the newtype pattern is a zero-cost abstraction, it can be used without any runtime overhead.
*/

struct Kilometers(i32);

trait FlightPlanner {
    fn list_flights(&self, from: Kilometers, to: Kilometers);
}

struct FlightPlannerImpl {}

impl FlightPlanner for FlightPlannerImpl {
    fn list_flights(&self, from: Kilometers, to: Kilometers) {
        println!("Listing flights from {} to {} kilometers", from.0, to.0);
    }
}

pub fn run() {
    let from = Kilometers(3);
    let to = Kilometers(5);
    let planner = FlightPlannerImpl {};
    planner.list_flights(from, to);
}
