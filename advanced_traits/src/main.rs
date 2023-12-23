mod associated_types;
mod default_generic_types;
mod fully_qualified_calling;
mod newtype_pattern;
mod operator_overload;
mod supertraits;

fn main() {
    println!("=== Associated Types ===");
    associated_types::run();

    println!("=== Default Generic Types ===");
    default_generic_types::run();

    println!("=== Operator Overload ===");
    operator_overload::run();

    println!("=== Fully Qualified Calling ===");
    fully_qualified_calling::run();

    println!("=== Supertraits ===");
    supertraits::run();

    println!("=== Newtype Pattern ===");
    newtype_pattern::run();
}
