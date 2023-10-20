/*
    In this example workspace, we have 3 member projects:
    - binary
    - some_lib
    - some_other_lib

    The some_other_lib project defines a struct LibraryStruct which just holds a number.
    The some_lib project defines a function multiply which multiplies two LibraryStructs and returns the result as a new LibraryStruct.
    This is the binary project, which uses both some_lib and some_other_lib.

    The Cargo.toml file contained in the root directory defines the workspace.
    Here, all members (sub-projects) are defined.

    Each member also has its own Cargo.toml file.
    This is the normal Cargo.toml file, like you would have in a normal project.
    But what makes it special is that you can define dependencies between the members.
    For an example, see the Cargo.toml files in the binary and some_lib directories.

    All members share the same Cargo.lock file, which is also located in the root directory.
    This means that all members use the same versions of dependencies.
    This is important, because otherwise you could end up with different versions of the same dependency in different members.
    This could lead to problems, because the members are compiled separately and then linked together.
    If one member uses a different version of a dependency than another member, the linker could not find the symbols of the dependency.

    Also, all members share the same target directory for compilation.
*/

use some_lib;
use some_other_lib;

fn main() {
    // Create two instances of the library struct
    let library_struct1 = some_other_lib::LibraryStruct { number: 3 };
    let library_struct2 = some_other_lib::LibraryStruct { number: 5 };

    // Multiply them with the multiply function from the library
    let result = some_lib::multiply(library_struct1, library_struct2);

    // Print the result
    println!("Result: {}", result.number);
}
