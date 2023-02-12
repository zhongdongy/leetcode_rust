use crate::common::Case;

pub fn use_cases() -> Vec<Case<i32, i32, i32>> {
    let mut cases: Vec<Case<i32, i32, i32>> = vec![];
    cases.push(Case::new(-2147483648, vec![0]));
    cases.push(Case::new(2147483647, vec![0]));
    cases.push(Case::new(0, vec![0]));
    cases.push(Case::new(123, vec![321]));
    cases.push(Case::new(-123, vec![-321]));
    cases.push(Case::new(120, vec![21]));
    cases.push(Case::new(900800, vec![8009]));
    cases.push(Case::new(2147457412, vec![0]));
    cases.push(Case::new(-2147483412, vec![-2143847412]));

    cases
}
