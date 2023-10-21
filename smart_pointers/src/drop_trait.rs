/*
    Implementing the drop trait on a type gives you a way to run some code whenever an instance of your type goes out of scope.
*/

pub fn run() {
    println!("=== Automatic drop ===");
    custom_smart_pointer_example();

    println!("=== Early manual drop ===");
    custom_smart_pointer_early_example();
}

struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: String) -> Self {
        println!("Creating a CustomSmartPointer with data `{}`...", data);
        let custom_smart_pointer = CustomSmartPointer { data };
        println!("Created a CustomSmartPointer. Moving ownership to the caller...");
        custom_smart_pointer
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn custom_smart_pointer_example() {
    let custom_smart_pointer = CustomSmartPointer::new(String::from("my stuff"));
    println!(
        "Received ownership of a CustomSmartPointer with data `{}`.",
        custom_smart_pointer.data
    );
    println!("CustomSmartPointer will now be dropped.");
}

/*
    In some cases, you might want to clean up a value early.
    One example is when using smart pointers that manage locks.
*/
fn custom_smart_pointer_early_example() {
    let custom_smart_pointer = CustomSmartPointer::new(String::from("other stuff"));
    println!(
        "Received ownership of a CustomSmartPointer with data `{}`.",
        custom_smart_pointer.data
    );
    println!("CustomSmartPointer will now be dropped.");
    drop(custom_smart_pointer);
    println!("CustomSmartPointer was dropped before the end of the function!");
}
