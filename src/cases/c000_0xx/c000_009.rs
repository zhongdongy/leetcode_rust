use crate::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<i32, bool, i32>> {
    let mut case_group: CaseGroup<i32, bool, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create, $val1,$val2);
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+)
        };
    }

    new!(
        // #1
        {123, false},
        {10, false},
        {1230, false},
        {123032, false},
        {123321, true},
        // #6
        {-123321, false},
        {1000030001, false},
        {1, true},
        {0, true},
        {121, true},
        // #11
        {213, false}
    );

    case_group.all()
}
