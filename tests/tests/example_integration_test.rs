/*
    Integration tests are entirely external to a library.
    They use a library in the same way any other external code would,
    using only the public API and potentially exercising multiple modules per test.
*/

use example; // Notice that despite the directory being called tests, we use example here, as defined in Cargo.toml

#[test]
fn test_add_externally() {
    assert_eq!(4, example::add(2, 2));
}
