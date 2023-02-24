//! # 问题描述
//!
//! 给定一个仅包含数字 `2-9` 的字符串，返回所有它能表示的字母组合。答案可以按**任意顺序**
//! 返回。
//! 
//! 给出数字到字母的映射如下（与电话按键相同）。注意 `1` 不对应任何字母。
//! 
//! ![](https://eastwind-cdn.dongs.xyz/image/20230224010445.png)
//! 
//! 示例 1：
//! 
//! ```plain
//! 输入：digits = "23"
//! 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
//! ```
//! 
//! 示例 2：
//! 
//! ```plain
//! 输入：digits = ""
//! 输出：[]
//! ```
//! 
//! 示例 3：
//! 
//! ```plain
//! 输入：digits = "2"
//! 输出：["a","b","c"]
//! ```
//! 
//! 提示：
//! 
//! - `0 $\leqslant$ digits.length $\leqslant$ 4`
//! - `digits[i]` 是范围 `['2', '9']` 的一个数字。
//! 
//! 来源: <https://leetcode.cn/problems/letter-combinations-of-a-phone-number/>

////////////////////////////////////////////////////////////////////////////////

/// 电话号码的字母组合
///
/// # 参数
/// * `digits` - 输入的数字序列
///
/// # 示例
///
/// ```rust
/// use leetcode_rust::problems_cn::p000_0xx::p000_017::letter_combinations;
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
