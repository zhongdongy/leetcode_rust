//! # 问题描述
//!
//! 编写一个函数来查找字符串数组中的最长公共前缀。
//! 
//! 如果不存在公共前缀，返回空字符串 `""`。
//! 
//! 示例 1：
//! 
//! ```plain
//! 输入：strs = ["flower","flow","flight"]
//! 输出："fl"
//! ```
//! 
//! 示例 2：
//! 
//! ```plain
//! 输入：strs = ["dog","racecar","car"]
//! 输出：""
//! 解释：输入不存在公共前缀。
//! ```
//! 
//! 提示：
//! 
//! - `1 $\leqslant$ strs.length $\leqslant$ 200`
//! - `0 $\leqslant$ strs[i].length $\leqslant$ 200`
//! - `strs[i]` 仅由小写英文字母组成
//! 
//! 来源：<https://leetcode.cn/problems/longest-common-prefix/>

////////////////////////////////////////////////////////////////////////////////

/// 最长公共前缀
///
/// # 参数
/// * `strs` - 输入字符串
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
