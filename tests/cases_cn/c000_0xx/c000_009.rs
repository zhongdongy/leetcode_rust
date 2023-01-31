use leetcode_rust::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<i32, bool, i32>> {
    let mut case_group: CaseGroup<i32, bool, i32> = CaseGroup::new();

    // #1
    new_case!(case_group.create, 123, false);
    new_case!(case_group.create, 10, false);
    new_case!(case_group.create, 1230, false);
    new_case!(case_group.create, 123032, false);
    new_case!(case_group.create, 123321, true);
    // #6
    new_case!(case_group.create, -123321, false);
    new_case!(case_group.create, 1000030001, false);
    new_case!(case_group.create, 1, true);
    new_case!(case_group.create, 0, true);
    new_case!(case_group.create, 121, true);
    // #11
    new_case!(case_group.create, 213, false);

    case_group.all()
}
