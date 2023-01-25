use super::super::cases_cn::c000_0xx::c000_005;
use super::super::cases_cn::c000_0xx::c000_006;
use leetcode_rust::problems_cn::p000_0xx::p000_005;
use leetcode_rust::problems_cn::p000_0xx::p000_006;

/// Test Problem 000_005: 最长回文子串
#[test]
fn p000_005_longest_palindrome() {
    for case in c000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((&case.input).to_string()));
    }
}

/// Test Problem 000_006: Z 字形变换
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
