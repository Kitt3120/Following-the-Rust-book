mod cool_module;

//Added afterwards:
use cool_module::part_of_cool_module;
use cool_module::part_of_cool_module as submodule;
use cool_module::part_of_cool_module::function_in_part_of_cool_module as that_one_function;
use cool_module::part_of_cool_module::function_in_part_of_cool_module_that_will_be_lifted as that_other_function;

fn main() {
    println!("Hello, world!");
    cool_module::cool_function();
    cool_module::part_of_cool_module::function_in_part_of_cool_module();
    cool_module::part_of_cool_module::function_in_part_of_cool_module_that_will_be_lifted();

    //After adding use statement:
    part_of_cool_module::function_in_part_of_cool_module();
    part_of_cool_module::function_in_part_of_cool_module_that_will_be_lifted();

    submodule::function_in_part_of_cool_module();
    submodule::function_in_part_of_cool_module_that_will_be_lifted();

    cool_module::function_in_part_of_cool_module_that_will_be_lifted();

    that_one_function();
    that_other_function();
}
