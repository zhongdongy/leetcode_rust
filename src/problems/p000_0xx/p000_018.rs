//! # Description
//!
//! Given an array `nums` of `n` integers, return an array of all the unique
//! quadruplets `[nums[a], nums[b], nums[c], nums[d]]` such that:
//!
//! - `0 $\\leqslant$ a, b, c, d < n`
//! - `a`, `b`, `c`, and `d` are distinct.
//! - `nums[a] + nums[b] + nums[c] + nums[d] == target`
//!
//! You may return the answer in any order.
//!
//! Example 1:
//!
//! ```plain
//! Input: nums = [1,0,-1,0,-2,2], target = 0
//! Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: nums = [2,2,2,2,2], target = 8
//! Output: [[2,2,2,2]]
//! ```
//!
//! Constraints:
//!
//! - `1 $\leqslant$ nums.length $\leqslant$ 200`
//! - `$-10^9 \leqslant$ nums[i] $\leqslant 10^9$`
//! - `$-10^9 \leqslant$ target $\leqslant 10^9$`
//!
//! Sources: <https://leetcode.com/problems/4sum/>

////////////////////////////////////////////////////////////////////////////////

/// 4Sum
///
/// # Arguments
/// * `nums` - input numbers
/// * `target` - a number taht every quadruplet should sum up to.
///
/// # Examples
///
/// ```rust
/// ```
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    four_sum_algorithm1(nums, target)
}

#[allow(dead_code)]
fn four_sum_algorithm1(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() <= 3 {
        return vec![];
    }

    let mut quadruplets = vec![];

    nums.sort();

    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = 0usize;
    let mut l = 0usize;

    loop {
        if i > nums.len() - 4 {
            // Not enough 4 digits, should break;
            break;
        }

        // Every time i updated, initial j to next pos of i.
        j = i + 1;
    }

    quadruplets
}

#[cfg(test)]
mod tests {}
