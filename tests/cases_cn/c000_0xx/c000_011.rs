use leetcode_rust::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<i32, i32, i32>> {
    let mut case_group: CaseGroup<i32, i32, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create_multi, $val1, $val2);
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+)
        };
    }

    new!(
        {vec![1, 8, 6, 2, 5, 4, 8, 3, 7],   49},
        {vec![2, 3, 12, 12, 1, 2],          12},
        {vec![1, 1],                        1}
    );

    case_group.all()
}
