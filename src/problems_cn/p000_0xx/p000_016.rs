//! # 问题描述
//!
//! 给你一个长度为 `n` 的整数数组 `nums` 和 一个目标值 `target`。请你从 `nums` 中选出三个
//! 整数，使它们的和与 `target` 最接近。
//!
//! 返回这三个数的和。
//!
//! 假定每组输入只存在恰好一个解。
//!
//! 示例 1：
//!
//! ```plain
//! 输入：nums = [-1,2,1,-4], target = 1
//! 输出：2
//! 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。
//! ```
//!
//! 示例 2：
//!
//! ```plain
//! 输入：nums = [0,0,0], target = 1
//! 输出：0
//! ```
//!
//! 提示：
//!
//! - `3 $\leqslant$ nums.length $\leqslant$ 1000`
//! - `-1000 $\leqslant$ nums[i] $\leqslant$ 1000`
//! - `$-10^4$ $\leqslant$ target $\leqslant$ $10^4$`
//!
//! 来源：<https://leetcode.cn/problems/3sum-closest/>

////////////////////////////////////////////////////////////////////////////////

/// 最接近的三数之和
///
/// # 参数
/// * `nums` - 传入数字序列
/// * `target` - 期望靠近的目标值
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    algorithm(nums, target)
}

#[allow(unused_assignments)]
fn algorithm(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted_nums = nums;
    sorted_nums.sort();

    let mut res = sorted_nums.iter().take(3).sum();
    if res >= target {
        return res;
    }

    let mut i = 0;
    let mut last_diff = ((target - res) as i32).abs();
    let mut this_diff = 0;
    while i <= sorted_nums.len() - 3 {
        let mut j = i + 1;

        let mut k = sorted_nums.len() - 1;

        while j < k {
            let temp = sorted_nums[i] + sorted_nums[j] + sorted_nums[k];
            if temp == target {
                return temp;
            }

            if temp <= target {
                j += 1;
                this_diff = target - temp;
            } else {
                k -= 1;
                this_diff = temp - target;
            }
            // 由于改用提前计算差值的办法，所以不会溢出。
            if this_diff <= last_diff {
                res = temp;
                last_diff = this_diff;
            }
        }

        i += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::three_sum_closest;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(three_sum_closest(vec![0, 1, 2], 0), 3);
    }
}
