/*
    Because of Rust's nature, the compiler needs to know a lot about the types of the variables in your code.
    One of the things it needs to know is the size of the types, so it can allocate the right amount of memory for them.

    However, in reality, there are some types whose size simply can't be known at compile time.
    For example, a str (not &str) is a dynamically sized type, because its size depends on the length of the string it contains.

    So how does Rust handle this?
    As it was already demonstrated many times, in Rust you use &str instead of str.
    So the actual data of the string is stored somewhere else, and the &str is just a reference to it.
    This way, the compiler knows the size of the type: it's the size of a pointer some metadata, like the length of the string.
    Now the compiler can allocate the right amount of memory for the &str, even if the length of the string is not known at compile time.
    Instead of &str, we can also use other pointers, like Box<str> or Rc<str>.

    Whether or not a size is known at compile can be expressed with the Sized trait.
    All types whose sizes are known at compile time implement this trait.
    For example, i32 implements Sized, because its size is known at compile time.
    If you create your own structs, they will also implement Sized by default.

    Want to hear a little plot twist?
    All trait objects in Rust are dynamically sized types.
    If you think about it, it makes sense:
    The size of a trait object depends on the size of the type that implements the trait.
    It could be (and probably is) implemented by many different types, all with different sizes.
    This is why you have to use Box<dyn Trait> instead of just the trait itself for trait objects.
    By doing this, you're moving the dynamically sized part onto the heap.
    The stack will only contain the pointer to it, whose size is known at compile time.
    In comparison, if you were using just the trait's type, it would be like using str instead of &str.

    The Sized trait is also automatically implemented for all generic type parameters.
    So if you define a function like this: fn foo<T>(), the compiler will treat it as if it was fn foo<T: Sized>().
    This is why you can't use a trait object as a generic type parameter, but a pointer to it is fine.
*/

trait DoSomething {
    fn do_something(&self);
}

// Size is 1*u32 = 4 bytes
struct OneNumber {
    number: u32,
}

impl DoSomething for OneNumber {
    fn do_something(&self) {
        println!("OneNumber has value: {}", self.number)
    }
}

// Size is 2*u32 = 8 bytes
struct TwoNumbers {
    number1: u32,
    number2: u32,
}

impl DoSomething for TwoNumbers {
    fn do_something(&self) {
        println!(
            "TwoNumbers has values: {} and {}",
            self.number1, self.number2
        )
    }
}

pub fn run() {
    let mut blyat_list = Vec::<Box<dyn DoSomething>>::new();

    blyat_list.push(Box::new(OneNumber { number: 1 }));
    blyat_list.push(Box::new(TwoNumbers {
        number1: 2,
        number2: 3,
    }));

    for blyat in blyat_list {
        blyat.do_something();
    }
}
