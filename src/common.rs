//! # Common Contents
//!
//! This module provide common structs, enums and functions used in solutions,
//! test cases, test suites.
//!
//! # 公共内容
//!
//! 此模块提供用于解法、测试用例、测试程序使用的公共 Struct、Enum 和函数。
//!

////////////////////////////////////////////////////////////////////////////////

use std::fmt;

/// Test case wrapper struct
///
/// # Generics
/// * `T` - type of test input
/// * `G` - type of expectations, must implement `PartialEq` and `Display` traits
/// * `P` - type of optional parameters
pub struct Case<T, G, P> {
    /// Input value of test case
    pub input: T,

    /// Optional parameters when executing test case
    pub params: Vec<P>,

    /// Expected values of given input
    pub values: Vec<G>,

    /// Test case index (for labeling)
    index: i32,
}

impl<T, G, P> Case<T, G, P>
where
    T: PartialEq + fmt::Display,
    G: PartialEq + fmt::Display,
    P: PartialEq + fmt::Display,
{
    /// Create new test case with no parameters
    ///
    /// # Arguments
    /// * `input` - test input
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new(input: T, values: Vec<G>) -> Case<T, G, P> {
        Case {
            input: input,
            params: vec![],
            values: values,
            index: 0,
        }
    }

    /// Create new test case with parameters
    ///
    /// # Arguments
    /// * `input` - test input
    /// * `params` - test parameters that vary among different cases
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new_params(input: T, params: Vec<P>, values: Vec<G>) -> Case<T, G, P> {
        Case {
            input: input,
            params: params,
            values: values,
            index: 0,
        }
    }

    /// Check if solution output matches any expectations
    ///
    /// # Arguments
    /// * `&self` - inmutable borrow to object itself
    /// * `result` - solution output
    ///
    /// ```
    /// use leetcode_rust::common::Case;
    /// let case:Case<String, String, i32> = Case::new(String::from("A"), vec![String::from("A")]);
    /// case.is_valid(String::from("A"))
    /// ```
    pub fn is_valid(&self, result: G) {
        if self.values.len() == 0 {
            assert!(false);
        }
        for val in self.values.iter() {
            if *val == result {
                return assert!(true);
            }
        }

        if self.values.len() == 1 {
            assert!(
                false,
                "[#{}] INPUT=`{}`, OUTPUT=`{}`, EXPECTATION=`{}`",
                self.index, &self.input, &result, self.values[0]
            );
        } else {
            assert!(false, "Result `{}` doesn't match any expectations", &result);
        }
    }

    pub fn label(&self) -> String {
        self.index.to_string()
    }
}

pub struct CaseGroup<T, G, P> {
    cases: Vec<Case<T, G, P>>,
    count: i32,
}

impl<T, G, P> CaseGroup<T, G, P> {
    pub fn new() -> CaseGroup<T, G, P> {
        CaseGroup {
            cases: vec![],
            count: 0,
        }
    }

    pub fn add(&mut self, mut case: Case<T, G, P>) {
        self.count += 1;
        case.index = self.count;
        self.cases.push(case);
    }

    pub fn all(self) -> Vec<Case<T, G, P>> {
        self.cases
    }
}
