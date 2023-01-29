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
//! ```plain
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
//! ```plain
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
//! ```plain
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
/// 
/// ```
/// use leetcode_rust::problems::p000_0xx::p000_008::my_atoi;
/// 
/// assert!(my_atoi(String::from("-12.5")) == -12);
/// ```
pub fn my_atoi(s: String) -> i32 {
    // Upper bound for possitive i32. The last element should be increased
    // if given string indicates a negative i32 (no sign symbol included).
    let mut threshold_val: [u8; 10] = [50, 49, 52, 55, 52, 56, 51, 54, 52, 55];

    // Indicates whether the given string starts with a negative integer.
    let mut is_negative = false;

    // Indicates existence of digits in given string.
    let mut has_digits = false;

    // Current index position during scanning of input string.
    let mut curr_idx: usize = 0;

    // Indicates whether scanned part should be treated as an integer.
    let mut is_started = false;

    // Temp value during looping and return value after looping complete.
    let mut val: i32 = 0;

    // Bytes form of input string. For faster comparison and computation.
    let s_bytes = s.as_bytes();

    // Indicates whether the scanned part correspond to an overflowed interger.
    let mut is_overflow = false;

    // In some cases, we should just ignore overflow detection because the
    // parsed digits are simply less than same digit in possitive / negative
    // overflow threshold value respectively.
    let mut is_overflow_ignored = false;

    // Used to check if given string starts with exactly same digits comparing
    // with threshold.
    let mut is_full_match = true;

    // A vector containing all parsed and valid digits.
    let mut val_vec: Vec<u8> = vec![];

    // Loop forever.
    // Looping through all characters in sequence or escape during looping are
    // controlled by additional flags.
    loop {
        // Guard condition, exit looping when no more characters to check.
        if curr_idx == s_bytes.len() {
            break;
        }

        if s_bytes[curr_idx] == 32 {
            // This is a whitespace character, check its position.
            if !is_started {
                // Leading whitespace, ignore it
                curr_idx += 1;
                continue;
            } else {
                // None leading whitespace, end of reading because we already
                // found some significant digits.
                break;
            }
        }

        if [43u8, 45u8].contains(&s_bytes[curr_idx]) {
            // Check positive and negative signs.
            if has_digits {
                // Signs after digits (even after 0 is not allowed)
                // e.g. `12-222`, `0+123`
                break;
            }
            if !is_started {
                // Should only adjust sign when this is the very first valid
                // symbol in the given string.
                if s_bytes[curr_idx] == 45 {
                    // Adjust flag and set new overflow threshold.
                    is_negative = true;
                    threshold_val = [50, 49, 52, 55, 52, 56, 51, 54, 52, 56];
                }
                // Setup flag to avoid `-+` sequences.
                is_started = true;
                curr_idx += 1;
                continue;
            }
        }

        // Now parse digit and detect overflow.
        if 48 <= s_bytes[curr_idx] && s_bytes[curr_idx] <= 57 {
            // Once a new digit found, update related flags to avoid malformed
            // sequences like: `0 123` and `123-6`.
            has_digits = true;
            is_started = true;

            if val == 0 && s_bytes[curr_idx] == 48 {
                // Skip leading zeros.
                curr_idx += 1;
                continue;
            }

            // Digits
            if val_vec.len() >= threshold_val.len() - 1 {
                // Only check overflow when parsed digits are the same or 1
                // digit shorter than threshold. Otherwise it could waste
                // execution time.

                // The following traversal checks:
                // 1. if parsed part has exact same digits as threshold;
                // 2. if parsed part should be ignored in future checking.
                for idx in 0..val_vec.len() {
                    if !is_overflow {
                        // Save time
                        if !is_overflow_ignored && val_vec[idx] > threshold_val[idx] {
                            // One digit is greater than threshold, no need to
                            // check later ones.
                            is_overflow = true;
                            is_full_match = false;
                            break;
                        } else if val_vec[idx] < threshold_val[idx] {
                            // One digit is smaller than threshold, means as
                            // long as its shorter than threshold, it cannot
                            // overflow.
                            // But it still needs testing for its length
                            // in a later step.
                            is_full_match = false;
                            is_overflow_ignored = true;
                        }
                    }
                }

                if !is_overflow {
                    // Test for current digit:
                    // 1. if it extends length of parsed number that causes
                    // overflow.
                    // 2. if it increases parsed number that causes overflow.
                    if val_vec.len() == threshold_val.len() - 1 {
                        // Check if adding this digit causes overflow
                        if is_full_match
                            && s_bytes[curr_idx] > threshold_val[threshold_val.len() - 1]
                        {
                            is_overflow = true;
                            break;
                        }
                    } else if val_vec.len() == threshold_val.len() {
                        // Check if extending parsed part causes overflow.
                        is_overflow = true;
                        break;
                    }
                }
            }
            if is_overflow {
                // Do NOT prepend current digit to parsed number.
                break;
            }
            val = val * 10 + (s_bytes[curr_idx] - 48) as i32;
            val_vec.push(s_bytes[curr_idx]);
            curr_idx += 1;
            continue;
        }

        break;
    }
    // The following steps simply determines what value to return by
    // overflowing flag and integer sign.
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
