//! # Description
//!
//! Implement the `myAtoi(string s)` function, which converts a string to a
//! 32-bit signed integer (similar to C/C++'s atoi function).
//!
//! The algorithm for myAtoi(string s) is as follows:
//!
//! Read in and ignore any leading whitespace.
//!
//! Check if the next character (if not already at the end of the string) is
//! '-' or '+'. Read this character in if it is either. This determines if the
//! final result is negative or positive respectively. Assume the result is
//! positive if neither is present.
//!
//! Read in next the characters until the next non-digit character or the end
//! of the input is reached. The rest of the string is ignored.
//! Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If
//! no digits were read, then the integer is 0. Change the sign as necessary
//! (from step 2).
//!
//! If the integer is out of the 32-bit signed integer range [-231, 231 - 1],
//! then clamp the integer so that it remains in the range. Specifically,
//! integers less than -231 should be clamped to -231, and integers greater
//! than 231 - 1 should be clamped to 231 - 1.
//! Return the integer as the final result.
//! Note:
//!
//! Only the space character ' ' is considered a whitespace character.
//! Do not ignore any characters other than the leading whitespace or the rest
//! of the string after the digits.
//!  
//!
//! Example 1:
//! ```ignore
//! Input: s = "42"
//! Output: 42
//! Explanation: The underlined characters are what is read in, the caret is the
//! current reader position.
//! Step 1: "42" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "42" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "42" ("42" is read in)
//!            ^
//! The parsed integer is 42.
//! Since 42 is in the range [-231, 231 - 1], the final result is 42.
//! ```
//!
//! Example 2:
//! ```ignore
//! Input: s = "   -42"
//! Output: -42
//! Explanation:
//! Step 1: "   -42" (leading whitespace is read and ignored)
//!             ^
//! Step 2: "   -42" ('-' is read, so the result should be negative)
//!              ^
//! Step 3: "   -42" ("42" is read in)
//!                ^
//! The parsed integer is -42.
//! Since -42 is in the range [-231, 231 - 1], the final result is -42.
//! ```
//!
//! Example 3:
//! ```
//! Input: s = "4193 with words"
//! Output: 4193
//! Explanation:
//! Step 1: "4193 with words" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "4193 with words" ("4193" is read in; reading stops because the next
//! character is a non-digit)
//!              ^
//! The parsed integer is 4193.
//! Since 4193 is in the range [-231, 231 - 1], the final result is 4193.
//! ```
//!
//! Constraints:
//!
//! - $0 \leqslant s.length \leqslant 200$
//! - s consists of English letters (lower-case and upper-case), digits (0-9),
//! ' ', '+', '-', and '.'.

////////////////////////////////////////////////////////////////////////////////

/// Convert string to 32-bit integer
///
/// # Argument
/// * `s` - input string
pub fn my_atoi(s: String) -> i32 {
    let mut threshold_val: [u8; 10] = [50, 49, 52, 55, 52, 56, 51, 54, 52, 55];
    let mut is_negative = false;
    let mut has_digits = false;
    let mut curr_idx: usize = 0;
    let mut is_started = false;
    let mut val: i32 = 0;
    let s_bytes = s.as_bytes();
    let mut is_overflow = false;
    let mut is_full_match = true;
    let mut val_vec: Vec<u8> = vec![];
    loop {
        if curr_idx == s_bytes.len() {
            break;
        }

        if s_bytes[curr_idx] == 32 {
            if !is_started {
                // Leading whitespace, ignore
                curr_idx += 1;
                continue;
            } else {
                // None leading whitespace, end of reading
                break;
            }
        }

        if !is_started && [43u8, 45u8].contains(&s_bytes[curr_idx]) {
            if has_digits {
                // Signs after digits (even 0 not allowed)
                break;
            }
            if s_bytes[curr_idx] == 45 {
                is_negative = true;
                threshold_val = [50, 49, 52, 55, 52, 56, 51, 54, 52, 56];
            }
            is_started = true;
            curr_idx += 1;
            continue;
        }

        if 48 <= s_bytes[curr_idx] && s_bytes[curr_idx] <= 57 {
            has_digits = true;
            if val == 0 && s_bytes[curr_idx] == 48 {
                // Skip leading zeros.
                curr_idx += 1;
                continue;
            }

            // Digits
            for idx in 0..val_vec.len() {
                if !is_overflow {
                    if val_vec[idx] > threshold_val[idx] {
                        is_overflow = true;
                        is_full_match = false;
                        break;
                    } else if val_vec[idx] < threshold_val[idx] {
                        is_full_match = false;
                    }
                }
            }
            if !is_overflow {
                // Check if adding this digit causes overflow
                if is_full_match || val_vec.len() == threshold_val.len() {
                    is_overflow = true;
                    break;
                }
            }

            val = val * 10 + (s_bytes[curr_idx] - 48) as i32;
            val_vec.push(s_bytes[curr_idx]);
            curr_idx += 1;
            continue;
        }

        break;
    }
    if is_overflow {
        if is_negative {
            -2147483648
        } else {
            2147483647
        }
    } else {
        if is_negative {
            -val
        } else {
            val
        }
    }
}
