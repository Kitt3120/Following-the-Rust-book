mod deduplication_by_function;
mod deduplication_by_generics;
mod deduplication_enums_by_generics;
mod deduplication_struct_by_generics;

fn main() {
    deduplication_by_function::bad();
    deduplication_by_function::good();
    println!("====================================================================================================");
    deduplication_by_generics::bad();
    deduplication_by_generics::good();
    println!("====================================================================================================");
    deduplication_struct_by_generics::bad();
    deduplication_struct_by_generics::good();
    println!("====================================================================================================");
    deduplication_enums_by_generics::bad();
    deduplication_enums_by_generics::good();
}

/*
    Performance of Code Using Generics
    You might be wondering whether there is a runtime cost when using generic type parameters.
    The good news is that using generic types won't make your program run any slower than it would with concrete types.
    Rust accomplishes this by performing monomorphization of the code using generics at compile time.
    Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
    In this process, the compiler does the opposite of the steps we used to create the generic function.
    The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.
*/
