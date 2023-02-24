//! # Description
//!
//! Given a string containing digits from `2-9` inclusive, return all possible
//! letter combinations that the number could represent. Return the answer in
//! any order.
//!
//! A mapping of digits to letters (just like on the telephone buttons) is
//! given below. Note that `1` does not map to any letters.
//!
//! ![](https://eastwind-cdn.dongs.xyz/image/20230224010445.png?w=256)
//!
//! Example 1:
//!
//! ```plain
//! Input: digits = "23"
//! Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: digits = ""
//! Output: []
//! ```
//!
//! Example 3:
//!
//! ```plain
//! Input: digits = "2"
//! Output: ["a","b","c"]
//! ```
//!
//! Constraints:
//!
//! - `0 $\leqslant$ digits.length $\leqslant$ 4`
//! - `digits[i]` is a digit in the range `['2', '9']`.
//!
//! Sources: <https://leetcode.com/problems/letter-combinations-of-a-phone-number/>

////////////////////////////////////////////////////////////////////////////////

/// Letter Combinations of a Phone Number
///
/// # Arguments
/// * `digits` - input digits
///
/// # Examples
///
/// ```rust
/// use leetcode_rust::problems::p000_0xx::p000_017::letter_combinations;
///
/// let input = String::from("2");
/// let output = letter_combinations(input);
/// assert_eq!(output, vec!["a", "b", "c"]);
/// ```
pub fn letter_combinations(digits: String) -> Vec<String> {
    // letter_combinations_by_match_control_flow(digits)
    letter_combinations_by_pre_allocation(digits)
    // letter_combinations_by_pre_allocation_v2(digits)
}

/// Match structure to get the letters for each digit
///
/// # Arguments
/// - `digits` - input digits
#[allow(dead_code)]
fn letter_combinations_by_match_control_flow(digits: String) -> Vec<String> {
    use std::str;
    let mut res: Vec<String> = vec![];
    for digit in digits.as_bytes() {
        let letters = match *digit {
            b'2' => "abc",
            b'3' => "def",
            b'4' => "ghi",
            b'5' => "jkl",
            b'6' => "mno",
            b'7' => "pqrs",
            b'8' => "tuv",
            b'9' => "wxyz",
            _ => "",
        }
        .as_bytes();

        if res.len() > 0 {
            let mut temp_res = vec![];
            for partial in res {
                for letter in letters {
                    let mut new_partial = partial.clone();
                    new_partial.push(*letter as char);
                    temp_res.push(new_partial);
                }
            }
            res = temp_res;
        } else {
            res = letters
                .iter()
                .map(|l| str::from_utf8(vec![*l].as_slice()).unwrap().to_string())
                .collect();
        }
    }

    res
}

/// Create a vector of bytes representing each digit in advance. Save memory usage
/// significantly.
///
/// # Arguments
/// - `digits` - input digits
#[allow(dead_code)]
fn letter_combinations_by_pre_allocation(digits: String) -> Vec<String> {
    use std::str;

    let digits_to_letters: Vec<&[u8]> =
        vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"]
            .iter()
            .map(|s| s.as_bytes())
            .collect();

    let mut res: Vec<String> = vec![];
    for digit in digits.as_bytes() {
        let letters = digits_to_letters[(*digit - b'2') as usize];

        if res.len() > 0 {
            let mut temp_res = vec![];
            for partial in res {
                for letter in letters {
                    let mut new_partial = partial.clone();
                    new_partial.push(*letter as char);
                    temp_res.push(new_partial);
                }
            }
            res = temp_res;
        } else {
            let mut temp: Vec<String> = letters
                .iter()
                .map(|l| str::from_utf8(vec![*l].as_slice()).unwrap().to_string())
                .collect();
            res.append(&mut temp);
        }
    }

    res
}

/// Create a vector of bytes representing each digit in advance. Save memory
/// usage significantly. Simplified expression but may cause a little more
/// memory consumption.
///
/// # Arguments
/// - `digits` - input digits
#[allow(dead_code)]
fn letter_combinations_by_pre_allocation_v2(digits: String) -> Vec<String> {
    use std::str;

    let digits_to_letters: Vec<&[u8]> =
        vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"]
            .iter()
            .map(|s| s.as_bytes())
            .collect();

    let mut res: Vec<String> = vec![];
    for digit in digits.as_bytes() {
        let letters = digits_to_letters[(*digit - b'2') as usize];

        res = if res.len() == 0 {
            letters
                .iter()
                .map(|l| str::from_utf8(&[*l]).unwrap().to_string())
                .collect()
        } else {
            let mut temp_res = vec![];
            for partial in res {
                for letter in letters {
                    temp_res.push(partial.clone() + str::from_utf8(&[*letter]).unwrap());
                }
            }
            temp_res
        };
    }

    res
}

#[cfg(test)]
mod tests {
    use super::letter_combinations;
    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(letter_combinations(String::from("")), Vec::<String>::new());
        assert_eq!(letter_combinations(String::from("2")), vec!["a", "b", "c"]);
    }
}
