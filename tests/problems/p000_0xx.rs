use crate::cases::c000_0xx::c000_005;
use crate::cases::c000_0xx::c000_006;
use crate::cases::c000_0xx::c000_007;
use crate::cases::c000_0xx::c000_008;
use crate::cases::c000_0xx::c000_009;
use crate::cases::c000_0xx::c000_010;
use crate::cases::c000_0xx::c000_011;
use crate::cases::c000_0xx::c000_012;
use crate::cases::c000_0xx::c000_013;
use leetcode_rust::problems::p000_0xx::p000_005;
use leetcode_rust::problems::p000_0xx::p000_006;
use leetcode_rust::problems::p000_0xx::p000_007;
use leetcode_rust::problems::p000_0xx::p000_008;
use leetcode_rust::problems::p000_0xx::p000_009;
use leetcode_rust::problems::p000_0xx::p000_010;
use leetcode_rust::problems::p000_0xx::p000_011;
use leetcode_rust::problems::p000_0xx::p000_012;
use leetcode_rust::problems::p000_0xx::p000_013;

/// Test Problem 000_005: Longest Palindrome Substring
#[test]
fn p000_005_longest_palindrome() {
    for case in c000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((case.input()).to_string()));
    }
}

/// Test Problem 000_006: ZigZag Conversion
#[test]
fn p000_006_zigzag_conversion() {
    for case in c000_006::use_cases() {
        case.is_valid(p000_006::zigzag_conversion(
            (case.input()).to_string(),
            case.params[0],
            Some(p000_006::Algorithm::STACK),
        ));
    }
}

/// Test Problem 000_007: Reverse Integer
#[test]
fn p000_007_reverse_integer() {
    for case in c000_007::use_cases() {
        case.is_valid(p000_007::reverse_integer(*case.input()));
    }
}

/// Test Problem 000_008: Convert String to Integer (atoi)
#[test]
fn p000_008_convert_string_to_integer() {
    for case in c000_008::use_cases() {
        case.is_valid(p000_008::my_atoi((case.input()).to_string()));
    }
}

/// Test Problem 000_009: Palindrome Number
#[test]
fn p000_009_is_palindrome_number() {
    for case in c000_009::use_cases() {
        case.is_valid(p000_009::is_palindrome(*case.input()));
    }
}

/// Test Problem 000_010: Regular Expression Matching
#[test]
fn p000_010_regular_expression_matching() {
    for case in c000_010::use_cases() {
        case.is_valid(p000_010::is_match(
            case.inputs[0].clone(),
            case.inputs[1].clone(),
        ));
    }
}

/// Test Problem 000_011: Container With Most Water
#[test]
fn p000_011_container_with_most_water() {
    for case in c000_011::use_cases() {
        case.is_valid(p000_011::max_area(case.inputs.to_vec()));
    }
}

/// Test Problem 000_012: Integer to Roman
#[test]
fn p000_012_integer_to_roman() {
    for case in c000_012::use_cases() {
        case.is_valid(p000_012::int_to_roman(*case.input()));
    }
}

/// Test Problem 000_013: Roman to Integer
#[test]
fn p000_013_integer_to_roman() {
    for case in c000_013::use_cases() {
        case.is_valid(p000_013::roman_to_int(case.input().clone()));
    }
}