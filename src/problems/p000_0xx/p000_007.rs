//! # Description
//!
//! Given a signed 32-bit integer `x`, return `x` with its digits reversed.
//! If reversing `x` causes the value to go outside the signed 32-bit integer
//! range \[-2^31, 2^31 - 1\], then return 0.
//!
//! Assume the environment does not allow you to store 64-bit integers (signed 
//! or unsigned).
//!
//!
//! | Example 1 |
//! | :-- |
//! | Input: x = 123 |
//! | Output: 321 |
//!
//! | Example 2 |
//! | :-- |
//! | Input: x = -123 |
//! | Output: -321 |
//! 
//! | Example 3 |
//! | :-- |
//! | Input: x = 120 |
//! | Output: 21 |
//!  
//!
//! Constraints:
//!
//! - `-2^31 <= x <= 2^31 - 1`
//!
//! Source: <https://leetcode.com/problems/reverse-integer/description/>

////////////////////////////////////////////////////////////////////////////////

/// Reverse an integer (32-bit long) and check for overflow.
///
/// Exchange front and end digits (not bits) one by one, return zero if
/// overflow
///
/// # Argument
/// * `x` - 32-bit signed integer to alter
///
/// ```
/// use leetcode_rust::problems::p000_0xx::p000_007::reverse_integer;
/// assert_eq!(reverse_integer(-2147483647), 0);
/// assert_eq!(reverse_integer(123), 321);
/// assert_eq!(reverse_integer(120), 21);
/// assert_eq!(reverse_integer(-123), -321);
/// ```
pub fn reverse_integer(x: i32) -> i32 {
    reverse_s1(x)
}

/// Reverse an integer (32-bit long) and check for overflow.
///
/// # Argument
/// * `x` - 32-bit signed integer to alter
fn reverse_s1(x: i32) -> i32 {
    let mut temp_stack: Vec<u8> = vec![];
    // Convert integer to digits.
    for ch in x.to_string().as_bytes() {
        if *ch != 45 {
            temp_stack.push(*ch);
        }
    }
    loop {
        // Remove trailing zeros if present
        match temp_stack.last() {
            Some(last_ch) => {
                if *last_ch == 48 && temp_stack.len() > 1 {
                    temp_stack.pop();
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    temp_stack.reverse();

    // 2147483647 for positive numbers
    let mut overflow_at_u8: [u8; 10] = [50, 49, 52, 55, 52, 56, 51, 54, 52, 55];
    let mut target: Vec<u8> = vec![];
    // Detect sign of input number and update overflow threshold if needed.
    if x < 0 {
        target.push('-' as u8);

        // 2147483648 for negative numbers (without sign)
        overflow_at_u8 = [50, 49, 52, 55, 52, 56, 51, 54, 52, 56];
    }

    // Check overflow regardless of the sign
    if temp_stack.len() >= overflow_at_u8.len() {
        for idx in 0..overflow_at_u8.len() {
            if temp_stack[idx] < overflow_at_u8[idx] {
                // If current digit is smaller than overflow threshold, no
                // need to test anymore.
                break;
            }
            if temp_stack[idx] > overflow_at_u8[idx] {
                // Previous digit (if exists) same between target and threshold,
                // If current digit of target is greater than threshold, then
                // overflow.
                return 0;
            }
        }
    }
    // Combine sign and digits
    target = [target, temp_stack].concat();
    String::from_utf8(target).unwrap().parse::<i32>().unwrap()
}
