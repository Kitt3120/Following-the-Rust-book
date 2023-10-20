use some_other_lib::{self, LibraryStruct};

pub fn multiply(library_struct1: LibraryStruct, library_struct2: LibraryStruct) -> LibraryStruct {
    LibraryStruct {
        number: library_struct1.number * library_struct2.number,
    }
}
