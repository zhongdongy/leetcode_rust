//! # Description
//!
//! Given an integer array `nums`, return all the triplets `[nums[i], nums[j],
//! nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
//!
//! Notice that the solution set must not contain duplicate triplets.
//!
//! Example 1:
//!
//! ```plain
//! Input: nums = [-1,0,1,2,-1,-4]
//! Output: [[-1,-1,2],[-1,0,1]]
//! Explanation:
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//! The distinct triplets are [-1,0,1] and [-1,-1,2].
//! Notice that the order of the output and the order of the triplets does not matter.
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: nums = [0,1,1]
//! Output: []
//! Explanation: The only possible triplet does not sum up to 0.
//! ```
//!
//! Example 3:
//!
//! ```plain
//! Input: nums = [0,0,0]
//! Output: [[0,0,0]]
//! Explanation: The only possible triplet sums up to 0.
//! ```
//!
//! Constraints:
//!
//! - `3 $\leqslant$ nums.length $\leqslant$ 3000`
//! - `-105 $\leqslant$ nums[i] $\leqslant$ 105`
//!
//! Sources: <https://leetcode.com/problems/3sum/>

////////////////////////////////////////////////////////////////////////////////

/// 3Sum
///
/// # Arguments
/// * `nums` - input number pairs
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

/// Sample algorithm implementation from LeetCode (fastest)
///
/// # Arguments
/// * `nums` - input numbers to check for triplets.
#[allow(dead_code)]
pub fn algorithm_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let len = nums.len();
    nums.sort_unstable();

    for i in 0..len {
        let left = nums[i];

        if i > 0 && nums[i - 1] == left {
            continue;
        }

        let mut middle = i + 1;
        let mut right = len - 1;

        while middle < right {
            let curr_sum = left + nums[middle] + nums[right];

            if curr_sum == 0 {
                res.push(vec![left, nums[middle], nums[right]]);
                middle += 1;

                while middle < right && nums[middle - 1] == nums[middle] {
                    middle += 1;
                }

                right -= 1;
            } else if curr_sum < 0 {
                middle += 1;
            } else {
                right -= 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
        assert_eq!(Vec::<Vec<i32>>::from(vec![]), three_sum(vec![0, 1, 1]));
        assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0]));
        assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0, 0]));
    }
}
