//! # 问题描述
//!
//! 给你一个整数数组 `nums` ，判断是否存在三元组 `[nums[i], nums[j], nums[k]]` 满足
//! `i != j`、`i != k` 且 `j != k` ，同时还满足 `nums[i] + nums[j] + nums[k] == 0`。
//! 请你返回所有和为 `0` 且不重复的三元组。
//! 
//! 注意：答案中不可以包含重复的三元组。
//! 
//! 示例 1：
//! 
//! ```plain
//! 输入：nums = [-1,0,1,2,-1,-4]
//! 输出：[[-1,-1,2],[-1,0,1]]
//! 解释：
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
//! 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
//! 注意，输出的顺序和三元组的顺序并不重要。
//! ```
//! 
//! 示例 2：
//! 
//! ```plain
//! 输入：nums = [0,1,1]
//! 输出：[]
//! 解释：唯一可能的三元组和不为 0 。
//! ```
//! 
//! 示例 3：
//! 
//! ```plain
//! 输入：nums = [0,0,0]
//! 输出：[[0,0,0]]
//! 解释：唯一可能的三元组和为 0 。
//! ``` 
//! 
//! 提示：
//! 
//! - `3 $\leqslant$ nums.length $\leqslant$ 3000`
//! - `$-10^{5} \leqslant$ nums[i] $\leqslant 10^{5}$`
//! 
//! 来源：<https://leetcode.cn/problems/3sum/>

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