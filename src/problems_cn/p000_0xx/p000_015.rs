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

/// 三数之和
///
/// # 参数
/// * `nums` - 输入数字序列
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    algorithm_2(nums)
}

/// Personal implementation
///
/// # Arguments
/// * `nums` - input numbers to check for triplets.
fn algorithm_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    if nums.len() >= 3 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut i = 0;
        while i < sorted_nums.len() - 1 {
            let a = sorted_nums[i];

            if i > 0 && sorted_nums[i - 1] == sorted_nums[i] {
                i += 1;
                continue;
            }

            let mut j = i + 1;

            while j < sorted_nums.len() - 1 {
                let b = sorted_nums[j];

                if a + b > 0 {
                    break;
                }

                if i + 1 < j && b == sorted_nums[j - 1] {
                    // Already searched last time, continue
                    j += 1;
                    continue;
                }

                let mut start = 0;
                let mut end = sorted_nums[(j + 1)..].len() - 1;

                while start < end {
                    let middle = (end + start) / 2;
                    if sorted_nums[(j + 1)..][middle] < -a - b {
                        start = middle + 1;
                    } else {
                        end = middle;
                    }
                }

                let k = j + 1 + start;

                let c = sorted_nums[k];

                if a + b + c == 0 {
                    res.push(vec![a, b, c]);
                }
                j += 1;
            }

            i += 1;
        }
    }

    res
}