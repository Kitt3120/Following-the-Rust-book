/*
    A trait can have a supertrait defined.
    This means that the trait can only be implemented on types that also implement the supertrait.
    This is similar to how a class can inherit from another class in other languages, just on a trait level.
    By doing this, the self type within the trait has the supertrait implemented and its methods can be called.
*/

trait Wolf {
    fn howl(&self);
}

trait Dog: Wolf {
    fn bark(&self);
}

struct WolflikeDogger;

impl Wolf for WolflikeDogger {
    fn howl(&self) {
        println!("Awoooooo!");
    }
}

impl Dog for WolflikeDogger {
    fn bark(&self) {
        // Here, self can be treated as a Wolf, because Wolf is a supertrait of Dog.
        println!("Bark!");
    }
}

pub fn run() {
    let dog = WolflikeDogger;
    dog.howl();
    dog.bark();
}
