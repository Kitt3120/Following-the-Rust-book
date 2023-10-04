mod deduplication_by_function;
mod deduplication_by_generics;

fn main() {
    deduplication_by_function::bad();
    deduplication_by_function::good();
    println!("====================================================================================================");
    deduplication_by_generics::bad();
    deduplication_by_generics::good();
}
