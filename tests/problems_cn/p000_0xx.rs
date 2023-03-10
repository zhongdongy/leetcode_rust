use leetcode_rust::{cases_cn::c000_0xx::*, problems_cn::p000_0xx::*};

/// 题目 000_005: 最长回文子串
#[test]
fn p000_005_longest_palindrome() {
    for case in c000_005::use_cases() {
        case.is_valid(p000_005::longest_palindrome((case.input()).to_string()));
    }
}

/// 题目 000_006: Z 字形变换
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

/// 题目 000_007: 整数反转
#[test]
fn p000_007_reverse_integer() {
    for case in c000_007::use_cases() {
        case.is_valid(p000_007::reverse_integer(*case.input()));
    }
}

/// 题目 000_008: 字符串转换整数 (atoi)
#[test]
fn p000_008_convert_string_to_integer() {
    for case in c000_008::use_cases() {
        case.is_valid(p000_008::my_atoi((case.input()).to_string()));
    }
}

/// 题目 000_009: 回文数
#[test]
fn p000_009_is_palindrome_number() {
    for case in c000_009::use_cases() {
        case.is_valid(p000_009::is_palindrome(*case.input()));
    }
}

/// 题目 000_010: 正则表达式匹配
#[test]
fn p000_010_regular_expression_matching() {
    for case in c000_010::use_cases() {
        case.is_valid(p000_010::is_match(
            case.inputs[0].clone(),
            case.inputs[1].clone(),
        ));
    }
}

/// 题目 000_011：盛最多水的容器
#[test]
fn p000_011_max_area() {
    for case in c000_011::use_cases() {
        case.is_valid(p000_011::max_area(case.inputs.to_vec()));
    }
}

/// 题目 000_012：整数转罗马数字
#[test]
fn p000_012_int_to_roman() {
    for case in c000_012::use_cases() {
        case.is_valid(p000_012::int_to_roman(*case.input()));
    }
}

/// 题目 000_013：罗马数字转整数
#[test]
fn p000_013_integer_to_roman() {
    for case in c000_013::use_cases() {
        case.is_valid(p000_013::roman_to_int(case.input().clone()));
    }
}

/// 题目 000_014：最长公共前缀
#[test]
fn p000_014_longest_common_prefix() {
    for case in c000_014::use_cases() {
        case.is_valid(p000_014::longest_common_prefix(case.inputs.to_vec()));
    }
}

/// 题目 000_015：三数之和
#[test]
fn p000_015_three_sum() {
    for case in c000_015::use_cases() {
        case.is_valid(p000_015::three_sum(case.input().clone()));
    }
}


/// 题目 000_016：最接近的三数之和
#[test]
fn p000_016_three_sum_closest() {
    for case in c000_016::use_cases() {
        case.is_valid(p000_016::three_sum_closest(
            case.inputs.clone(),
            case.params[0],
        ));
    }
}

/// 题目 000_017: 电话号码的字母组合
#[test]
fn p000_017_letter_combinations() {
    for case in c000_017::use_cases() {
        case.is_valid(p000_017::letter_combinations(case.input().clone()));
    }
}