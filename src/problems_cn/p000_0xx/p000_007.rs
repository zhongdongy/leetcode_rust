//! # 题目说明
//! 给你一个 32 位的有符号整数 `x` ，返回将 `x` 中的数字部分反转后的结果。
//!
//! 如果反转后整数超过 32 位的有符号整数的范围 $[−2^{31},  2^{31} − 1]$ ，就返回 0。
//!
//! 假设环境不允许存储 64 位整数（有符号或无符号）。
//!
//! | 示例 1 |
//! | :-- |
//! | 输入：x = 123 |
//! | 输出：321 |
//!
//! | 示例 2 |
//! | :-- |
//! | 输入：x = -123 |
//! | 输出：-321 |
//!
//! | 示例 3 |
//! | :-- |
//! | 输入：x = 120 |
//! | 输出：21 |
//!
//! | 示例 4 |
//! | :-- |
//! | 输入：x = 0 |
//! | 输出：0 |
//!
//! 提示：
//!
//! - $-2^{31} \leqslant x \leqslant 2^{31} - 1$
//!
//! 来源：<https://leetcode.cn/problems/reverse-integer>

////////////////////////////////////////////////////////////////////////////////

/// 32 位整型反转的同时检查溢出
///
///
/// # 参数
/// * `x` - 32 位有符号整数
///
/// ```
/// use leetcode_rust::problems_cn::p000_0xx::p000_007::reverse_integer;
/// assert_eq!(reverse_integer(-2147483647), 0);
/// assert_eq!(reverse_integer(123), 321);
/// assert_eq!(reverse_integer(120), 21);
/// assert_eq!(reverse_integer(-123), -321);
/// ```
pub fn reverse_integer(x: i32) -> i32 {
    reverse_s1(x)
}

/// 解法一：32 位整型反转的同时检查溢出
///
/// # 参数
/// * `x` - 32 位有符号整数
fn reverse_s1(x: i32) -> i32 {
    let mut temp_stack: Vec<u8> = vec![];
    // 将无符号数转换为数字数组
    for ch in x.to_string().as_bytes() {
        if *ch != 45 {
            temp_stack.push(*ch);
        }
    }
    loop {
        // 去除结尾的数字 0
        match temp_stack.last() {
            Some(last_ch) => {
                if *last_ch == 48 && temp_stack.len() > 1 {
                    temp_stack.pop();
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    temp_stack.reverse();

    // 正数的溢出阈值为 2147483647（包含符号）
    let mut overflow_at_u8: [u8; 10] = [50, 49, 52, 55, 52, 56, 51, 54, 52, 55];
    let mut target: Vec<u8> = vec![];
    // 检测输入值的符号并据此更新判断溢出的阈值
    if x < 0 {
        target.push('-' as u8);

        // 负数的溢出阈值为 2147483648（不包含符号）
        overflow_at_u8 = [50, 49, 52, 55, 52, 56, 51, 54, 52, 56];
    }

    // 在无符号的状态下检查溢出
    if temp_stack.len() >= overflow_at_u8.len() {
        for idx in 0..overflow_at_u8.len() {
            if temp_stack[idx] < overflow_at_u8[idx] {
                // 如果某一位数值比溢出值要小，则没有必要检查后续的各位数值
                break;
            }
            if temp_stack[idx] > overflow_at_u8[idx] {
                // 上一个数值（如果存在）位置的目标值和阈值相同，
                // 如果当前位的目标值大于阈值则表示溢出
                return 0;
            }
        }
    }
    // 结合符号和数字
    target = [target, temp_stack].concat();
    String::from_utf8(target).unwrap().parse::<i32>().unwrap()
}
