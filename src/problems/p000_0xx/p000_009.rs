//! # Description
//!
//! Given an integer `x`, return `true` if `x` is a
//! palindrome, and `false` otherwise.
//!
//! Example 1:
//!
//! ```plain
//! Input: x = 121
//! Output: true
//! Explanation: 121 reads as 121 from left to right and from right to left.
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: x = -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//! ```
//!
//! Example 3:
//!
//! ```plain
//! Input: x = 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//! ```
//!  
//! Constraints:
//!
//! $-2^{31} \leqslant x \leqslant 2^{31} - 1$
//!  
//! Follow up: Could you solve it without converting the integer to a string?
//!
//! Source: <https://leetcode.com/problems/palindrome-number/>

////////////////////////////////////////////////////////////////////////////////

/// Check if a given input is a palindrome number
///
/// # Arguments
/// * `x` - input number
///
/// # Examples
/// ```
/// use leetcode_rust::problems::p000_0xx::p000_009::is_palindrome;
///
/// assert!(is_palindrome(12321) == true);
/// assert!(is_palindrome(-12321) == false);
/// ```
pub fn is_palindrome(number: i32) -> bool {
    // alg_1(number)
    alg_2(number)
}

/// Algorithm 1: check digits using double-sided queue.
///
/// # Arguments
/// * `number` - input number to check
#[allow(dead_code)]
fn alg_1(mut number: i32) -> bool {
    if number < 0 {
        return false;
    }
    let mut temp: Vec<i32> = vec![];

    while number >= 10 {
        temp.push(number % 10);
        if number >= 10 {
            number = number / 10;
        }
    }
    temp.push(number);

    if temp.len() == 2 {
        return temp[0] == temp[1];
    }

    let mut idx_start = 0;
    let mut idx_end = temp.len() - 1;

    loop {
        if idx_end == idx_start {
            return true;
        }

        if idx_end - idx_start == 1 {
            return temp[idx_end] == temp[idx_start];
        }

        if temp[idx_end] != temp[idx_start] {
            return false;
        }
        idx_end -= 1;
        idx_start += 1;
    }
}

/// Algorithm 2: check digits fast subtraction.
///
/// # Arguments
/// * `number` - input number to check
#[allow(dead_code)]
fn alg_2(mut number: i32) -> bool {
    if number < 0 {
        return false;
    }

    // One digit shorter than actual length.
    let mut digits = (number as f32).log10().floor() as usize;
    if digits == 0 {
        return true;
    }

    loop {
        number = (number - (number % 10) * 10i32.pow(digits as u32)) / 10;
        if number < 0 || digits <= 1 {
            break;
        }

        digits -= 2;
    }

    number == 0
}
