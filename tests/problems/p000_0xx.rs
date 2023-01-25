use super::super::cases::c000_0xx::c000_005;
use super::super::cases::c000_0xx::c000_006;
use leetcode_rust::problems::p000_0xx::p000_005;
use leetcode_rust::problems::p000_0xx::p000_006;

/// Test Problem 000_005: Longest Palindrome
#[test]
fn p000_005_longest_palindrome() {
    for case in c000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((&case.input).to_string()));
    }
}

/// Test Problem 000_006: ZigZag Conversion
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
