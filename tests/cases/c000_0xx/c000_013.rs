use leetcode_rust::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<String, i32, i32>> {
    let mut case_group: CaseGroup<String, i32, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create, $val1, $val2);
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+)
        };
    }

    new!(
            {"I",               1},
            {"XI",              11},
            {"XXXII",           32},
            {"LVIII",           58},
            {"XCVIII",          98},
            {"CXXI",            121},
            {"CXXIII",          123},
            {"MI",              1001},
            {"MCDXLIV",         1444},
            {"MCDXCI",          1491},
            {"MDXCI",           1591},
            {"MCMXCIV",         1994},
            {"MMMCMXCVIII",     3998},
            {"MMMCMXCIX",       3999}
        );

    case_group.all()
}
