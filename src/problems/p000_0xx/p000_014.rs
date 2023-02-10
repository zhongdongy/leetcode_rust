//! # Description
//!
//! Write a function to find the longest common prefix string amongst an array of strings.
//!
//! If there is no common prefix, return an empty string `""`.
//!
//! Example 1:
//!
//! ```plain
//! Input: strs = ["flower","flow","flight"]
//! Output: "fl"
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: strs = ["dog","racecar","car"]
//! Output: ""
//! Explanation: There is no common prefix among the input strings.
//! ```
//! 
//! Constraints:
//!
//! - `1 $\leqslant$ strs.length $\leqslant$ 200`
//! - `0 $\leqslant$ strs[i].length $\leqslant$ 200`
//! - `strs[i]` consists of only lowercase English letters.
//!
//! Sources: <https://leetcode.com/problems/longest-common-prefix/>

////////////////////////////////////////////////////////////////////////////////

/// Longest Common Prefix
///
/// # Arguments
/// * `strs` - input strings
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix: Vec<u8> = vec![];

    for idx_0 in 0..strs[0].len() {
        let ch = strs[0].as_bytes()[idx_0];
        for idx_n in 1..strs.len() {
            if strs[idx_n].len() == idx_0 || ch != strs[idx_n].as_bytes()[idx_0] {
                // Early exit
                return String::from_utf8(prefix).unwrap();
            }
        }
        prefix.push(ch);
    }

    String::from_utf8(prefix).unwrap()
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix() {
        let res = longest_common_prefix(
            vec!["abcd", "a", ""]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(res, "");
        
        let res = longest_common_prefix(
            vec!["abcd", "a", "adc"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(res, "a");
    }
}
