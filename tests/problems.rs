mod cases;
use leetcode_rust::problems::p000_005;
use leetcode_rust::problems::p000_006;

/// Test Problem 000_005: Longest Palindrome
#[test]
fn test_p000_005() {
    for case in cases::p000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((&case.input).to_string()));
    }
}

/// Test Problem 000_006: ZigZag Conversion
#[test]
fn test_p000_006() {
    for case in cases::p000_006::use_cases() {
        case.is_valid(p000_006::zigzag_conversion(
            (&case.input).to_string(),
            case.params[0],
            Some(p000_006::Algorithm::STACK),
        ));
    }
}
