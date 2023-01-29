use leetcode_rust::common::{Case, CaseGroup};

pub fn use_cases() -> Vec<Case<String, i32, i32>> {
    let mut case_group: CaseGroup<String,i32,i32> = CaseGroup::new();
    case_group.add(Case::new(String::from("-12"), vec![-12]));
    case_group.add(Case::new(String::from("  -12"), vec![-12]));
    case_group.add(Case::new(String::from("-  12"), vec![0]));
    case_group.add(Case::new(String::from("+12"), vec![12]));
    case_group.add(Case::new(String::from("1.2"), vec![1]));
    case_group.add(Case::new(String::from("1200"), vec![1200]));
    case_group.add(Case::new(String::from("-  +12"), vec![0]));
    case_group.add(Case::new(String::from("-1 +12"), vec![-1]));
    case_group.add(Case::new(String::from("-1.+12"), vec![-1]));
    case_group.add(Case::new(String::from("2147483647"), vec![2147483647]));
    case_group.add(Case::new(String::from("2147483648"), vec![2147483647]));
    case_group.add(Case::new(String::from("21474836471"), vec![2147483647]));
    case_group.add(Case::new(String::from("-2147483649"), vec![-2147483648]));
    case_group.add(Case::new(String::from("-6 with words"), vec![-6]));
    case_group.add(Case::new(String::from("-91283472332"), vec![-2147483648]));
    case_group.add(Case::new(String::from("-21474836460"), vec![-2147483648]));
    case_group.add(Case::new(String::from("21474836460"), vec![2147483647]));
    case_group.add(Case::new(String::from("  0000000000012345678"),vec![12345678],));
    case_group.add(Case::new(String::from("00000-42a1234"), vec![0]));
    case_group.add(Case::new(String::from("2147483646"), vec![2147483646]));

    case_group.all()
}
