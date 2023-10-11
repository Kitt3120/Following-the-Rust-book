/*
    While Rusts type system and borrow checker are great for preventing memory bugs,
    it can't interpret the logic of your program to ensure it's working as intended.
    This is where tests come in.
    You can't test every possible input to your program,
    but you can test the most common ones and some edge cases you can think of.
    By doing this, you can cover parts of your program to ensure they're working as intended.
*/

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn matches(a: i32, b: i32) -> bool {
    a == b
}

/*
    It's convention to put unit tests in the same file as the code they're testing.
    You can organize them into a tests module.
*/

#[cfg(test)] //This tells the compiler to only compile this module when running tests.
mod tests {
    use crate::matches;

    #[test] // This tells cargo that this is a test.
    fn test_add_positive_positive() {
        assert_eq!(super::add(10, 5), 15);
    }

    #[test]
    fn test_add_positive_negative() {
        assert_eq!(super::add(10, -5), 5);
    }

    #[test]
    fn test_add_negative_positive() {
        assert_eq!(super::add(-10, 5), -5);
    }

    #[test]
    fn test_add_negative_negative() {
        assert_eq!(super::add(-10, -5), -15);
    }

    #[test]
    fn test_subtract_positive_positive() {
        assert_eq!(super::subtract(10, 5), 5);
    }

    #[test]
    fn test_subtract_positive_negative() {
        assert_eq!(super::subtract(10, -5), 15);
    }

    #[test]
    fn test_subtract_negative_positive() {
        assert_eq!(super::subtract(-10, 5), -15);
    }

    #[test]
    fn test_subtract_negative_negative() {
        assert_eq!(super::subtract(-10, -5), -5);
    }

    #[test]
    fn test_multiply_positive_positive() {
        assert_eq!(super::multiply(10, 5), 50);
    }

    #[test]
    fn test_multiply_positive_negative() {
        assert_eq!(super::multiply(10, -5), -50);
    }

    #[test]
    fn test_multiply_negative_positive() {
        assert_eq!(super::multiply(-10, 5), -50);
    }

    #[test]
    fn test_multiply_negative_negative() {
        assert_eq!(super::multiply(-10, -5), 50);
    }

    #[test]
    fn test_divide_positive_positive() {
        assert_eq!(super::divide(10, 5), 2);
    }

    #[test]
    fn test_divide_positive_negative() {
        assert_eq!(super::divide(10, -5), -2);
    }

    #[test]
    fn test_divide_negative_positive() {
        assert_eq!(super::divide(-10, 5), -2);
    }

    #[test]
    fn test_divide_negative_negative() {
        assert_eq!(super::divide(-10, -5), 2);
    }

    // You can also test if values are not equal.

    #[test]
    fn test_add_positive_positive_not_equal() {
        assert_ne!(super::add(10, 5), 10);
    }

    // Or if a value is true.

    #[test]
    fn test_matches_true() {
        assert!(matches(5, 5));
    }

    #[test]
    fn test_matches_false() {
        assert!(!matches(5, 10));
    }

    /*
        A test will fail if it panics.
        But by adding the should_panic attribute,
        you can specify that a test should panic,
        so it will pass if it panics.
    */
    #[test]
    #[should_panic]
    fn panic() {
        panic!("This test will fail!");
        // The assert macros will also panic if they fail,
        // which lets the test fail.
    }

    /*
        You can specify custom messages for when a test fails.
        You can also specify a message to be expected for the should_panic attribute.
        Try changing the message to see what happens.
    */
    #[test]
    #[should_panic(expected = "5 did not equal 10!")]
    fn custom_messages() {
        assert_eq!(5, 10, "5 did not equal 10!");
    }

    /*
       Should you have to handle Optionals in your test,
       you can let your test return Result<(), String> instead of ().
       This will let you use the ? operator to return early if the optional is None.
    */
    #[test]
    fn optional_valid() -> Result<(), String> {
        let optional = Some(5);
        let value = optional.ok_or("Optional was None!")?;
        assert_eq!(value, 5);
        Ok(())
    }

    #[test]
    fn optional_invalid() -> Result<(), String> {
        let optional: Option<i32> = None;
        let value = optional.ok_or("Optional was None!");
        assert!(value.is_err());
        Ok(())
    }
}
