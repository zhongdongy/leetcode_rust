use crate::{
    common::{Case, CaseGroup},
    new_case,
};

pub fn use_cases() -> Vec<Case<String, bool, i32>> {
    let mut case_group: CaseGroup<String, bool, i32> = CaseGroup::new();

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
        {vec!["a", "a"],                true},
        {vec!["a", "b"],                false},
        {vec!["a", "."],                true},
        {vec!["a", "ab*"],              true},
        {vec!["a", "a*bc.*"],           false},
        {vec!["a", ".*..a*"],           false},
        {vec!["aa", "a"],               false},
        {vec!["aa", "a*"],              true},
        {vec!["ab", "a*"],              false},
        {vec!["ab", ".*"],              true},
        {vec!["ab", ".*c"],             false},
        {vec!["ab", ".a"],              false},
        {vec!["ab", ".b"],              true},
        {vec!["ab", "b*"],              false},
        {vec!["ab", "bc"],              false},
        {vec!["abc", ".*"],             true},
        {vec!["abc", "a*bc"],           true},
        {vec!["abc", "a.*c"],           true},
        {vec!["abc", "a.c"],            true},
        {vec!["abc", "ac*c"],           false},
        {vec!["abc", "ac*bc"],          true},
        {vec!["aab", "a*"],             false},
        {vec!["adebb", ".*bbb*"],       true},
        {vec!["bbbba", ".*a*a"],        true},
        {vec!["adebbb", ".*bbb*"],      true},
        {vec!["aabcdde", "a*bc.*"],     true},
        {vec!["adebbbcbbb", ".*bbb*"],  true},
        {vec!["adebbbcbbb", ".*bb*"],   true},
        {vec!["adebbbcbbb", ".*b*"],    true}, 
        {vec!["cbaacacaaccbaabcb", "c*b*b*.*ac*.*bc*a*"],                true},
        {vec!["acaabbaccbbacaabbbb", "a*.*b*.*a*aa*a*"],                 false},
        {vec!["abcaaaaaaabaabcabac", ".*ab.a.*a*a*.*b*b*"],              true},
        {vec!["aaaaaaaaaaaaaaaaaaab", "a*a*aaaaa*a*a*a*a*a*a*a*a*a*a*"], false},
        {vec!["aaaaaaaaaaaaaaaaaaab", "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*"], false}
    );

    case_group.all()
}
