//! # 问题描述
//!
//! 给你一个整数 `x` ，如果 `x` 是一个回文整数，返回 `true` ；否则，返回 `false` 。
//! 
//! 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//! 
//! 例如，`121` 是回文，而 `123` 不是。
//!  
//! 
//! 示例 1：
//! 
//! ```plain
//! 输入：x = 121
//! 输出：true
//! ```
//! 
//! 示例 2：
//! 
//! ```plain
//! 输入：x = -121
//! 输出：false
//! 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
//! ```
//! 
//! 示例 3：
//! 
//! ```plain
//! 输入：x = 10
//! 输出：false
//! 解释：从右向左读, 为 01 。因此它不是一个回文数。
//! ```
//! 
//! 提示：
//! 
//! - $-2^{31} \leqslant x \leqslant 2^{31} - 1$
//! 
//! 来源：<https://leetcode.cn/problems/palindrome-number>

////////////////////////////////////////////////////////////////////////////////

/// 检查传入参数是不是一个回文数
///
/// # 参数
/// * `x` - 输入参数
///
/// # 举例
/// ```
/// use leetcode_rust::problems_cn::p000_0xx::p000_009::is_palindrome;
///
/// assert!(is_palindrome(12321) == true);
/// assert!(is_palindrome(-12321) == false);
/// ```
pub fn is_palindrome(number: i32) -> bool {
    // alg_1(number)
    alg_2(number)
}

/// 算法 1: 使用双端队列.
///
/// # 参数
/// * `number` - 待判定的数
#[allow(dead_code)]
fn alg_1(mut number: i32) -> bool {
    if number < 0 {
        return false;
    }
    let mut temp: Vec<i32> = vec![];

    while number >= 10 {
        temp.push(number % 10);
        if number >= 10 {
            number = number / 10;
        }
    }
    temp.push(number);

    if temp.len() == 2 {
        return temp[0] == temp[1];
    }

    let mut idx_start = 0;
    let mut idx_end = temp.len() - 1;

    loop {
        if idx_end == idx_start {
            return true;
        }

        if idx_end - idx_start == 1 {
            return temp[idx_end] == temp[idx_start];
        }

        if temp[idx_end] != temp[idx_start] {
            return false;
        }
        idx_end -= 1;
        idx_start += 1;
    }
}

/// 算法 2: 按数位快速相减.
///
/// # 参数
/// * `number` - 待判定的数
#[allow(dead_code)]
fn alg_2(mut number: i32) -> bool {
    if number < 0 {
        // 所有的负数都不是回文数
        return false;
    }

    // Log10 得到的值是真实的数字位数 -1.
    let mut digits = (number as f32).log10().floor() as usize;
    if digits == 0 {
        return true;
    }

    loop {
        number = (number - (number % 10) * 10i32.pow(digits as u32)) / 10;
        if number < 0 || digits <= 1 {
            break;
        }

        digits -= 2;
    }

    number == 0
}
