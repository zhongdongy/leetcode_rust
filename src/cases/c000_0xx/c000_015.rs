use std::fs;

use crate::{
    common::{Case, CaseGroup},
    new_case, vec2d,
};

use serde_json;

use crate::models::case::*;

pub fn use_cases() -> Vec<Case<Vec<i32>, Vec<Vec<i32>>, i32>> {
    let mut case_group: CaseGroup<Vec<i32>, Vec<Vec<i32>>, i32> = CaseGroup::new();

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
        {vec![-1,0,1,2,-1,-4],           vec2d![[-1,-1,2],[-1,0,1]]},
        {vec![0,1,1],                    vec![]},
        {vec![0,0,0],                    vec2d![[0,0,0]]},
        {vec![0,0,0,0],                  vec2d![[0,0,0]]},
        {vec![-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0],  vec2d![[-5,1,4],[-4,0,4],[-4,1,3],[-2,-2,4],[-2,1,1],[0,0,0]]},
        {[0;3000].to_vec(),            vec2d![[0,0,0]]}
    );

    // Load large cases from extras.

    if let Ok(json_content) = fs::read_to_string("extra/test_cases/data_three_sum.json") {
        let extra_case_group: TestCaseGroup<Vec<i32>, Vec<Vec<i32>>> =
            serde_json::from_str(json_content.as_str()).unwrap();

        for case in extra_case_group.cases {
            case_group.create(case.input, vec![case.expectation]);
        }

        println!("Test cases from extras loaded!");
    } else {
        eprintln!("Unable to load extras!")
    }

    case_group.all()
}
