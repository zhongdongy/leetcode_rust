use super::super::cases_cn::c000_0xx::c000_005;
use super::super::cases_cn::c000_0xx::c000_006;
use super::super::cases_cn::c000_0xx::c000_007;
use super::super::cases_cn::c000_0xx::c000_008;
use leetcode_rust::problems_cn::p000_0xx::p000_005;
use leetcode_rust::problems_cn::p000_0xx::p000_006;
use leetcode_rust::problems_cn::p000_0xx::p000_007;
use leetcode_rust::problems_cn::p000_0xx::p000_008;

/// 题目 000_005: 最长回文子串
#[test]
fn p000_005_longest_palindrome() {
    for case in c000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((&case.input).to_string()));
    }
}

/// 题目 000_006: Z 字形变换
#[test]
fn p000_006_zigzag_conversion() {
    for case in c000_006::use_cases() {
        case.is_valid(p000_006::zigzag_conversion(
            (&case.input).to_string(),
            case.params[0],
            Some(p000_006::Algorithm::STACK),
        ));
    }
}

/// 题目 000_007: 整数反转
#[test]
fn p000_007_reverse_integer() {
    for case in c000_007::use_cases() {
        case.is_valid(p000_007::reverse_integer(case.input));
    }
}

/// 题目 000_008: 字符串转换整数 (atoi)
#[test]
fn p000_008_convert_string_to_integer() {
    for case in c000_008::use_cases() {
        case.is_valid(p000_008::my_atoi((&case.input).to_string()));
    }
}
