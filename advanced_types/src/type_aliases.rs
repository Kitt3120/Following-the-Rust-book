/*
    Instead of using the newtype pattern, we can also resort to type aliases.
    While we can achieve a similar effect, type aliases do not provide the same level of safety as the newtype pattern.
    As they are just an alias for an existing type, they are interchangeable with the type they are aliasing.
*/

use std::error::Error;

type Kilometers = i32;

trait FlightPlanner {
    fn list_flights(&self, from: Kilometers, to: Kilometers);
}

struct FlightPlannerImpl {}

impl FlightPlanner for FlightPlannerImpl {
    fn list_flights(&self, from: Kilometers, to: Kilometers) {
        println!("Listing flights from {} to {} kilometers", from, to);
    }
}

pub fn run() {
    let from = 3; // i32 value!
    let to = 5; // i32 value!
    let planner = FlightPlannerImpl {};
    planner.list_flights(from, to); // Notice that we are passing i32 values!
}

/*
    So then why use type aliases?
    Type aliases are useful when we want to reduce repetition.
    Especially when using generic types, type aliases can be used to make the code more readable.
*/

trait GenericFlightPlanner<T> {
    fn list_flights(&self, from: T, to: T);
}

type _KilometersFlightPlanner = dyn GenericFlightPlanner<Kilometers>;

// Even more worth:
type _Result<T> = std::result::Result<T, dyn Error>;
type _HeapStuff = Box<dyn Error + Send + Sync>;

trait HeapStuffThings {
    fn takes_long_type(&self, stuff: _HeapStuff);
    fn returns_long_type(&self) -> _HeapStuff;
}
