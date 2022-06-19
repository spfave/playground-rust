// LESSON 10.5: Doc Tests
/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```rust
/// let result = lib_demo::div(10, 5);
/// assert_eq!(result, 2);
/// ```
///
/// # Example #2: 6 / 2 = 3
///
/// ```rust
/// let result = lib_demo::div(6, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// let result = lib_demo::div(3, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```rust
/// let result = lib_demo::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```rust
/// let result = lib_demo::sub(6, 9);
/// assert_eq!(result, -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

// LESSON 10.7: Integration Tests
