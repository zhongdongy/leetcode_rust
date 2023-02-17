//! # Description
//!
//! Given an integer array nums of length `n` and an integer `target`,
//! find three integers in nums such that the sum is closest to `target`.
//!
//! Return the _sum of the three integers_.
//!
//! You may assume that each input would have exactly one solution.
//!
//! Example 1:
//!
//! ```plain
//! Input: nums = [-1,2,1,-4], target = 1
//! Output: 2
//! Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: nums = [0,0,0], target = 1
//! Output: 0
//! Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//! ```
//!
//! Constraints:
//!
//! - `3 $\leqslant$ nums.length $\leqslant$ 500`
//! - `-1000 $\leqslant$ nums[i] $\leqslant$ 1000`
//! - `$-10^4$ $\leqslant$ target $\leqslant$ $10^4$`
//!
//! Sources: <https://leetcode.com/problems/3sum-closest/>

////////////////////////////////////////////////////////////////////////////////

/// 3Sum Closest
///
/// # Arguments
/// * `nums` - input number pairs
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
            // Should not be possible to overflow.
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
