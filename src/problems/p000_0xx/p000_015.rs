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
    let mut res: Vec<Vec<i32>> = vec![];

    if nums.len() >= 3 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut i = 0;
        while i < sorted_nums.len() - 1 {
            let a = sorted_nums[i];

            if i <= sorted_nums.len() - 3 && a == sorted_nums[i + 1] && a == sorted_nums[i + 2] {
                if a == 0 {
                    if !triplet_exists(&res, &0, &0, &0) {
                        res.push(vec![0, 0, 0]);
                    }
                }
                if sorted_nums.len() > i + 4 {
                    let next_different =
                        i + 1 + bin_search(&sorted_nums[(i + 3)..].to_vec(), &(a + 1));
                    i = next_different;
                    continue;
                }
            }

            let mut j = i + 1;

            while j < sorted_nums.len() - 1 {
                let b = sorted_nums[j];

                if a + b > 0 {
                    j += 1;
                    continue;
                }

                let k = j + 1 + bin_search(&sorted_nums[(j + 1)..].to_vec(), &(-a - b));

                let c = sorted_nums[k];

                if a + b + c == 0 {
                    if !triplet_exists(&res, &a, &b, &c) {
                        res.push(vec![a, b, c]);
                    }
                }
                j += 1;
            }

            i += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::bin_search;
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

    #[test]
    fn test_bin_search() {
        assert_eq!(bin_search(&vec![0, 1, 2], &2), 2);
        assert_eq!(bin_search(&vec![0, 1, 2, 3, 4], &5), 4);
        assert_eq!(bin_search(&vec![0, 1, 2, 3, 4], &4), 4);
        assert_eq!(bin_search(&vec![0, 0, 1, 2, 3], &0), 0);
        assert_eq!(bin_search(&vec![0, 0, 1, 1, 3], &1), 2);
    }
}

pub fn bin_search(v: &Vec<i32>, n: &i32) -> usize {
    let mut start = 0;
    let mut end = v.len() - 1;

    while start < end {
        let middle = (end + start) / 2;
        if v[middle] < *n {
            start = middle + 1;
        } else {
            end = middle;
        }
    }

    start
}

pub fn triplet_exists(res: &Vec<Vec<i32>>, a: &i32, b: &i32, _c: &i32) -> bool {
    let mut exists = false;
    if res.len() > 0 {
        let mut test_idx = res.len() - 1;
        loop {
            let temp = &res[test_idx];
            if temp[0] != *a {
                break;
            }
            if temp[0] == *a && temp[1] == *b {
                exists = true;
                break;
            }
            if test_idx == 0 {
                break;
            }
            test_idx -= 1;
        }
    }
    exists
}
