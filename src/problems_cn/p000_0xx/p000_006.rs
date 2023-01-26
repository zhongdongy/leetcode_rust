//! # 题目描述
//! 
//! 将一个给定字符串 `s` 根据给定的行数 `numRows` ，以从上往下、从左到右进行 Z 字形排列。
//! 
//! 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
//! 
//! ```plain
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! ```
//! 
//! 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。
//! 
//! 请你实现这个将字符串进行指定行数变换的函数：
//! 
//! `string convert(string s, int numRows);`
//! 
//! | 示例 1 |
//! | :-- |
//! | 输入：s = "PAYPALISHIRING", numRows = 3 |
//! | 输出："PAHNAPLSIIGYIR" |
//! 
//! | 示例 2 |
//! | :-- |
//! | 输入：s = "PAYPALISHIRING", numRows = 4 |
//! | 输出："PINALSIGYAHRPI" |
//! 
//! 解释：
//! 
//! ```plain
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//! ```
//! 
//! | 示例 3 |
//! | :-- |
//! | 输入： s = "A", numRows = 1 |
//! | 输出："A" |
//! 
//! 提示：
//! 
//! - `1 <= s.length <= 1000`
//! - `s` 由英文字母（小写和大写）、',' 和 '.' 组成
//! - `1 <= numRows <= 1000`
//! 
//! 来源：<https://leetcode.cn/problems/zigzag-conversion>

////////////////////////////////////////////////////////////////////////////////

/// 用于选择要执行的算法的 Enum
pub enum Algorithm {
    /// 使用栈数组实现的字形变换
    STACK = 0,

    /// 使用二维数组实现的字形变换
    MATRIX = 1,
}

/// Z 字形变换
///
/// 可以通过第三个参数选择要使用的算法
///
/// ### Arguments
/// * `s` - 等待变换的字符串
/// * `n_rows` - 变换后的行数
///
/// ```
/// use leetcode_rust::problems_cn::p000_0xx::p000_006::zigzag_conversion;
/// let mut result_value = zigzag_conversion(String::from("PAYPALISHIRING"), 1, None);
/// assert_eq!(result_value, String::from("PAYPALISHIRING"));
/// result_value = zigzag_conversion(String::from("PAYPALISHIRING"), 2, None);
/// assert_eq!(result_value, String::from("PYAIHRNAPLSIIG"));
/// result_value = zigzag_conversion(String::from("PAYPALISHIRING"), 3, None);
/// assert_eq!(result_value, String::from("PAHNAPLSIIGYIR"));
/// ```
pub fn zigzag_conversion(s: String, n_rows: i32, alg: Option<Algorithm>) -> String {
    match alg.unwrap_or(Algorithm::STACK) {
        Algorithm::MATRIX => convert_s1(s, n_rows as usize),
        Algorithm::STACK => convert_s2(s, n_rows),
    }
}

#[cfg(test)]
#[test]
fn test_zigzag_conversion() {
    assert!(zigzag_conversion(String::from("PAPAL"), 1, None) == String::from("PAPAL"));
}

/// Z 字形变换（解法一）
///
/// 使用二维矩阵设计的算法
///
/// ### Arguments
/// * `s` - 等待变换的字符串
/// * `n_cols` - 变换后的列数
#[allow(unused_assignments)]
fn convert_s1(s: String, n_cols: usize) -> String {
    let mut cur_row = 0u16;
    let mut cur_col = 0u16;
    let mut n_rows = 1;

    // Considering max string length 1_000 ASCII characters, minimum 1 column,
    // max row count would be 1_000.
    let mut arr = vec![0u8; n_cols * 1000];
    let mut direction = 0; // 0 for GO, 1 for TURN
    let mut conv_idx = 0usize;
    while conv_idx < s.len() {
        // Set value of current pos
        arr[cur_row as usize * n_cols + cur_col as usize] = s.as_bytes()[conv_idx];
        conv_idx += 1;
        // Calculate next coordinate and direction
        if cur_col == 0 {
            if n_cols > 1 {
                if direction == 1 {
                    direction = 0;
                }
                cur_col += 1;
            } else {
                // Move to next row directly because only 1 column needed.
                cur_row += 1;
                n_rows += 1;
            }
        } else {
            if direction == 0 {
                if cur_col < n_cols as u16 - 1 {
                    // Only move to the right
                    cur_col += 1;
                } else {
                    // Last position, change direction and move to next row
                    direction = 1;
                    cur_col -= 1;
                    cur_row += 1;
                    n_rows += 1;
                }
            } else {
                // Move to left and below
                cur_col -= 1;
                cur_row += 1;
                n_rows += 1;
            }
        }
    }

    conv_idx = 0;
    let mut output = vec![0u8; s.len()];
    cur_row = 0;
    cur_col = 0;
    let mut str_idx = 0;
    while conv_idx < s.len() {
        // Update value if not 0u8
        str_idx = cur_row as usize * n_cols + cur_col as usize;
        if arr[str_idx] != 0u8 {
            output[conv_idx] = arr[str_idx];
            conv_idx += 1;
        }
        // Change position.
        if cur_row < n_rows - 1 {
            cur_row += 1;
        } else {
            cur_row = 0;
            cur_col += 1;
        }
    }
    String::from_utf8(output.to_vec()).unwrap()
}

#[cfg(test)]
#[test]
fn test_conversion_s1() {
    assert!(convert_s1(String::from("PAPAL"), 1) == String::from("PAPAL"));
}

/// Z 字形变换（解法二）
///
/// 使用栈数组设计的算法
///
/// ### Arguments
/// * `s` - 等待变换的字符串
/// * `n_rows` - 变换后的行数
fn convert_s2(s: String, num_rows: i32) -> String {
    let mut rows = vec![String::new(); num_rows as usize];

    let mut cur_row: i32 = 0;
    let mut go_down: bool = true;
    for val in s.bytes() {
        rows[cur_row as usize].push(val as char);
        if !go_down && cur_row == 0 {
            go_down = true;
        } else if go_down && cur_row == num_rows - 1 {
            go_down = false;
        }

        if go_down {
            cur_row += 1;
        } else {
            cur_row -= 1;
        }

        cur_row = i32::min(num_rows - 1, cur_row);
        cur_row = i32::max(0, cur_row);
    }

    rows.join("").to_string()
}

#[cfg(test)]
#[test]
fn test_conversion_s2() {
    assert!(convert_s2(String::from("PAPAL"), 2) == String::from("PPLAA"));
}
