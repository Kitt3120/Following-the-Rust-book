mod lifetime_functions;
mod lifetime_methods;
mod lifetime_static;
mod lifetime_structs;

fn main() {
    lifetime_functions::introduce();
    lifetime_structs::introduce();
    lifetime_methods::introduce();
    lifetime_static::introduce();
}
