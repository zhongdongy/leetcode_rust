//! # Description
//! 
//! Given a string `s`, return the longest palindromic substring in `s`.
//! 
//! | Example 1 |
//! | :-- |
//! | Input: s = "babad" |
//! | Output: "bab" |
//! 
//! Explanation: "aba" is also a valid answer.
//! 
//! | Example 2 |
//! | :-- |
//! | Input: s = "cbbd" |
//! | Output: "bb" |
//!  
//! Constraints:
//! - `1 <= s.length <= 1000`
//! - `s` consist of only digits and English letters.
//! 
//! Source: <https://leetcode.com/problems/longest-palindromic-substring/>

////////////////////////////////////////////////////////////////////////////////

/// Find longest palindrome of a given string.
///
/// It's possible to have multiple correct answer.
/// See `tests/cases/c000_0xx/c000_005.rs` for more info.
/// 
/// # Arguments
/// * `s` - original string to search.
///
/// # Examples
/// ```
/// use leetcode_rust::problems::p000_0xx::p000_005::longest_palindrome;
/// let mut result_value = longest_palindrome(String::from("abbab"));
/// assert_eq!(result_value, String::from("abba"));
/// ```
pub fn longest_palindrome(s: String) -> String {
    by_array_index(&s).to_string()
}

#[cfg(test)]
#[test]
fn test_longest_palindrome() {
    assert!(longest_palindrome(String::from("abbbabbbac")) == String::from("abbbabbba"));
}

#[allow(unused_assignments)]
fn by_array_index(s: &str) -> &str {
    let b = s.as_bytes();
    if b.len() == 1 {
        return s;
    }

    let mut cur_longest_start_idx = 0;
    let mut cur_longest_end_idx = 0;
    let mut ite = 1;
    let mut cur_start_idx = 0;
    let mut cur_end_idx = 0;
    let mut should_repeat = false;
    while ite <= b.len() - 1 || should_repeat {
        cur_start_idx = ite - 1;
        cur_end_idx = ite;
        if should_repeat {
            if ite < b.len() - 1 {
                cur_end_idx = ite + 1;
            }
            ite += 1;
        }
        should_repeat = !should_repeat;
        while cur_start_idx > 0 && cur_end_idx < b.len() - 1 && b[cur_end_idx] == b[cur_start_idx] {
            cur_start_idx -= 1;
            cur_end_idx += 1;
        }
        if b[cur_end_idx] != b[cur_start_idx]
            && cur_end_idx - cur_start_idx > 2
            && b[cur_end_idx - 1] == b[cur_start_idx + 1]
        {
            cur_end_idx -= 1;
            cur_start_idx += 1;
        } else if b[cur_end_idx] != b[cur_start_idx] {
            continue;
        }
        if cur_end_idx - cur_start_idx > cur_longest_end_idx - cur_longest_start_idx {
            cur_longest_end_idx = cur_end_idx;
            cur_longest_start_idx = cur_start_idx;
        }
    }
    &s[cur_longest_start_idx..(cur_longest_end_idx + 1)]
}

#[cfg(test)]
#[test]
fn test_by_array_index() {
    assert!(by_array_index("QQ") == String::from("QQ"));
    assert!(by_array_index("QAQ") == String::from("QAQ"));
}
