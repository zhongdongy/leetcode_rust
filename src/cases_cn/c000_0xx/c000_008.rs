use crate::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<String, i32, i32>> {
    let mut case_group: CaseGroup<String, i32, i32> = CaseGroup::new();

    // #1
    new_case!(case_group.create, "-12", -12);
    new_case!(case_group.create, "  -12", -12);
    new_case!(case_group.create, "-  12", 0);
    new_case!(case_group.create, "+12", 12);
    new_case!(case_group.create, "1.2", 1);
    // #6
    new_case!(case_group.create, "1200", 1200);
    new_case!(case_group.create, "-  +12", 0);
    new_case!(case_group.create, "-1 +12", -1);
    new_case!(case_group.create, "-1.+12", -1);
    new_case!(case_group.create, "2147483647", 2147483647);
    // #11
    new_case!(case_group.create, "2147483648", 2147483647);
    new_case!(case_group.create, "21474836471", 2147483647);
    new_case!(case_group.create, "-2147483649", -2147483648);
    new_case!(case_group.create, "-6 with words", -6);
    new_case!(case_group.create, "-91283472332", -2147483648);
    // #16
    new_case!(case_group.create, "-21474836460", -2147483648);
    new_case!(case_group.create, "21474836460", 2147483647);
    new_case!(case_group.create, "  0000000000012345678", 12345678);
    new_case!(case_group.create, "00000-42a1234", 0);
    new_case!(case_group.create, "2147483646", 2147483646);
    // #21
    new_case!(case_group.create, "42", 42);
    new_case!(case_group.create, "   -42", -42);
    new_case!(case_group.create, "4193 with words", 4193);
    new_case!(case_group.create, "0  123", 0);
    new_case!(case_group.create, "with words 121", 0);
    // #26
    new_case!(case_group.create, "-+12", 0);

    case_group.all()
}
