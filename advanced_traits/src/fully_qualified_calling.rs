/*
    When working with a complex system of structs and traits, it can happen that
    you have two traits implementing the same method for the same type.
    Or an impl block for a struct that implements a method of the same name as a trait.

    This works but takes some care when calling the method.
    The below example shows how to call the methods you want to call.
*/

trait Dog {
    fn howl(&self);
    fn non_method();
}

trait Wolf {
    fn howl(&self);
    fn non_method();
}

struct Doggo;

impl Doggo {
    fn howl(&self) {
        println!("Awooo! (from Doggo)");
    }

    fn non_method() {
        println!("Non method (from Doggo)");
    }
}

impl Dog for Doggo {
    fn howl(&self) {
        println!("Awooo! (from Dog)");
    }

    fn non_method() {
        println!("Non method (from Dog)");
    }
}

impl Wolf for Doggo {
    fn howl(&self) {
        println!("Awooo! (from Wolf)");
    }

    fn non_method() {
        println!("Non method (from Wolf)");
    }
}

pub fn run() {
    let doggo = Doggo;

    // By calling the method on the struct, we call the method defined in the impl block for the struct.
    doggo.howl();

    // By specifying the trait and using the howl method as if it were a static method, we call trait's method.
    Dog::howl(&doggo);

    // This works for Wolf as well, obviously.
    Wolf::howl(&doggo);

    // This also works when calling it on the struct.
    Doggo::howl(&doggo);

    // You can also cast the struct to the trait and call the method on the trait.
    <Doggo as Dog>::howl(&doggo);
    <Doggo as Wolf>::howl(&doggo);

    // All this works well for methods, but what about non-methods?
    Doggo::non_method();
    //Dog::non_method(); // Doesn't work: There's no receiver to call the method on.
    //Dog::non_method(doggo); // Doesn't work: There's no receiver to call the method on and the method doesn't take an argument.
    <Doggo as Dog>::non_method();
    <Doggo as Wolf>::non_method();
}
