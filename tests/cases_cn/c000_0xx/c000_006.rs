use leetcode_rust::common::Case;

pub fn use_cases() -> Vec<Case<String, String, i32>> {
    let mut cases: Vec<Case<String, String, i32>> = vec![];
    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![1],
        vec![String::from("PAYPALISHIRING")],
    ));

    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![2],
        vec![String::from("PYAIHRNAPLSIIG")],
    ));

    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![3],
        vec![String::from("PAHNAPLSIIGYIR")],
    ));

    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![4],
        vec![String::from("PINALSIGYAHRPI")],
    ));

    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![5],
        vec![String::from("PHASIYIRPLIGAN")],
    ));

    cases.push(Case::new_params(
        String::from("PAYPALISHIRING"),
        vec![6],
        vec![String::from("PRAIIYHNPSGAIL")],
    ));

    cases.push(Case::new_params(
        String::from("P"),
        vec![1],
        vec![String::from("P")],
    ));

    cases.push(Case::new_params(
        String::from("PA"),
        vec![2],
        vec![String::from("PA")],
    ));

    cases.push(Case::new_params(
        String::from(".PAY"),
        vec![2],
        vec![String::from(".APY")],
    ));

    cases
}
