use leetcode_rust::common::Case;

pub fn use_cases() -> Vec<Case<String, i32>> {
    let mut cases: Vec<Case<String, i32>> = vec![];
    cases.push(Case::new(String::from("a"), vec![String::from("a")]));

    cases.push(Case::new(
        String::from("abcba"),
        vec![String::from("abcba")],
    ));
    cases.push(Case::new(
        String::from("babac"),
        vec![String::from("bab"), String::from("aba")],
    ));
    cases.push(Case::new(
        String::from("bbbbb"),
        vec![String::from("bbbbb")],
    ));
    cases.push(Case::new(
        String::from("bccacc"),
        vec![String::from("ccacc")],
    ));
    cases.push(Case::new(String::from("cc"), vec![String::from("cc")]));
    cases.push(Case::new(String::from("cdd"), vec![String::from("dd")]));
    cases.push(Case::new(
        String::from("adddddcbc"),
        vec![String::from("ddddd")],
    ));
    cases.push(Case::new(
        String::from("adddcbc"),
        vec![String::from("ddd"), String::from("cbc")],
    ));

    cases
}
