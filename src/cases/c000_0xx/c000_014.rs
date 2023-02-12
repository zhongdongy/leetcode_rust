use crate::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<String, String, i32>> {
    let mut case_group: CaseGroup<String, String, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create_multi, $val1, String::from($val2));
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+)
        };
    }

    new!(
        {vec![""],                          ""},
        {vec!["",""],                       ""},
        {vec!["a","","a"],                  ""},
        {vec!["ab","a","a"],                "a"},
        {vec!["abc","abc","abc"],           "abc"},
        {vec!["abbc","c","bc"],             ""}
    );

    case_group.all()
}
