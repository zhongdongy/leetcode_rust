use crate::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<String, Vec<String>, i32>> {
    let mut case_group: CaseGroup<String, Vec<String>, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, })=>{
            new_case!(case_group.create, $val1, vec![]);
        };
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create, $val1, Vec::from($val2).iter().map(|s| s.to_string()).collect());
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+,) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+);
        };
        ({$val1:expr, }, $({$val3:expr, $val4:expr}),+,) => {
            new!({$val1, });
            new!($({$val3, $val4}),+,);
        };
        ({$val1:expr, }, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, });
            new!($({$val3, $val4}),+,);
        };
    }

    new!(
        {"", },
        {"23", ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]},
        {"2", ["a", "b", "c"]}
    );

    case_group.all()
}
