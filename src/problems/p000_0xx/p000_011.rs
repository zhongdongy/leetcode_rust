//! # Description
//!
//! You are given an integer array height of length n. There are n vertical
//! lines drawn such that the two endpoints of the ith line are `(i, 0)` and
//! `(i, height[i])`.
//!
//! Find two lines that together with the x-axis form a container, such that
//! the container contains the most water.
//!
//! Return the maximum amount of water a container can store.
//!
//! Notice that you may not slant the container.
//!
//! Example 1:
//!
//! ![Image](/images/question_11.jpg)
//!
//! ```plain
//! Input: height = [1,8,6,2,5,4,8,3,7]
//! Output: 49
//! Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
//! In this case, the max area of water (blue section) the container can contain is 49.
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: height = [1,1]
//! Output: 1
//! ```
//!
//! Constraints:
//!
//! - `n == height.length`
//! - `2 $\leqslant$ n $\leqslant$ $10^5$`
//! - `0 $\leqslant$ height[i] $\leqslant$ $10^4$`
//!
//! Sources: <https://leetcode.com/problems/container-with-most-water/description/>

////////////////////////////////////////////////////////////////////////////////

/// Container With Most Water
///
/// # Arguments
/// * `height` - array of wall heights
pub fn max_area(height: Vec<i32>) -> i32 {
    alg_1(height)
}

use core::cmp::max;

/// Traverse and find the the region which contains most water
/// 
/// # Arguments
/// * `height` - array of wall heights
fn alg_1(height: Vec<i32>) -> i32 {
    let mut sidx: usize = 0;
    let mut eidx = height.len() - 1;
    let mut area_max = 0;
    let area = |x: usize, y: usize| -> i32 {
        return if height[x] >= height[y] {
            height[y]
        } else {
            height[x]
        } * (if x >= y { x - y } else { y - x }) as i32;
    };

    loop {
        if sidx == eidx {
            break;
        }
        area_max = max(area(sidx, eidx), area_max);
        let mut moved = false;
        if height[sidx] <= height[eidx] {
            while sidx < eidx {
                if height[sidx + 1] <= height[sidx] {
                    sidx += 1;
                    moved = true;
                } else {
                    break;
                }
            }
            if !moved {
                sidx += 1;
            }
        } else {
            while sidx < eidx {
                if height[eidx - 1] <= height[eidx] {
                    eidx -= 1;
                    moved = true;
                } else {
                    break;
                }
            }
            if !moved {
                eidx -= 1;
            }
        }
    }

    area_max
}
