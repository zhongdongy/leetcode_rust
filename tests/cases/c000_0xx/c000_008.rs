use leetcode_rust::common::{Case, CaseGroup};

pub fn use_cases() -> Vec<Case<String, i32, i32>> {
    let mut case_group: CaseGroup<String, i32, i32> = CaseGroup::new();

    // #1
    case_group.create("-12", vec![-12]);
    case_group.create("  -12", vec![-12]);
    case_group.create("-  12", vec![0]);
    case_group.create("+12", vec![12]);
    case_group.create("1.2", vec![1]);
    // #6
    case_group.create("1200", vec![1200]);
    case_group.create("-  +12", vec![0]);
    case_group.create("-1 +12", vec![-1]);
    case_group.create("-1.+12", vec![-1]);
    case_group.create("2147483647", vec![2147483647]);
    // #11
    case_group.create("2147483648", vec![2147483647]);
    case_group.create("21474836471", vec![2147483647]);
    case_group.create("-2147483649", vec![-2147483648]);
    case_group.create("-6 with words", vec![-6]);
    case_group.create("-91283472332", vec![-2147483648]);
    // #16
    case_group.create("-21474836460", vec![-2147483648]);
    case_group.create("21474836460", vec![2147483647]);
    case_group.create("  0000000000012345678", vec![12345678]);
    case_group.create("00000-42a1234", vec![0]);
    case_group.create("2147483646", vec![2147483646]);
    // #21
    case_group.create("42", vec![42]);
    case_group.create("   -42", vec![-42]);
    case_group.create("4193 with words", vec![4193]);
    case_group.create("0  123", vec![0]);
    case_group.create("with words 121", vec![0]);
    // #26
    case_group.create("-+12", vec![0]);

    case_group.all()
}
