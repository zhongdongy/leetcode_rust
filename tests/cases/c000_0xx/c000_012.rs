use leetcode_rust::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<i32, String, i32>> {
    let mut case_group: CaseGroup<i32, String, i32> = CaseGroup::new();

    macro_rules! new {
        ({$val1:expr, $val2:expr})=>{
            new_case!(case_group.create, $val1, String::from($val2));
        };
        ({$val1:expr, $val2:expr}, $({$val3:expr, $val4:expr}),+) => {
            new!({$val1, $val2});
            new!($({$val3, $val4}),+)
        };
    }

    new!(
            {1,     "I"},
            {11,    "XI"},
            {32,    "XXXII"},
            {58,    "LVIII"},
            {98,    "XCVIII"},
            {121,   "CXXI"},
            {123,   "CXXIII"},
            {1001,  "MI"},
            {1444,  "MCDXLIV"},
            {1491,  "MCDXCI"},
            {1591,  "MDXCI"},
            {1994,  "MCMXCIV"},
            {3998,  "MMMCMXCVIII"},
            {3999,  "MMMCMXCIX"}
        );

    case_group.all()
}
