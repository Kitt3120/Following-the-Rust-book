mod dynamically_sized_types;
mod never_type;
mod newtype_for_safety;
mod type_aliases;

fn main() {
    println!("=== Newtype for safety ===");
    newtype_for_safety::run();

    println!("=== Type aliases ===");
    type_aliases::run();

    println!("=== Never type ===");
    never_type::run();

    println!("=== Dynamically sized types ===");
    dynamically_sized_types::run();
}
