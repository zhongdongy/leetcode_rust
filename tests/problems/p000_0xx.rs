use crate::cases::c000_0xx::c000_005;
use crate::cases::c000_0xx::c000_006;
use crate::cases::c000_0xx::c000_007;
use crate::cases::c000_0xx::c000_008;
use leetcode_rust::problems::p000_0xx::p000_005;
use leetcode_rust::problems::p000_0xx::p000_006;
use leetcode_rust::problems::p000_0xx::p000_007;
use leetcode_rust::problems::p000_0xx::p000_008;

/// Test Problem 000_005: Longest Palindrome Substring
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

/// Test Problem 000_007: Reverse Integer
#[test]
fn p000_007_reverse_integer() {
    for case in c000_007::use_cases() {
        case.is_valid(p000_007::reverse_integer(case.input));
    }
}

/// Test Problem 000_008: Convert String to Integer (atoi)
#[test]
fn p000_008_convert_string_to_integer() {
    for case in c000_008::use_cases() {
        // if case.label() == String::from("11") {
            case.is_valid(p000_008::my_atoi((&case.input).to_string()));
        // }
    }
}
