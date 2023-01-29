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

use std::fmt::{self, Display};

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

    /// Returns the index value in String form.
    pub fn label(&self) -> String {
        self.index.to_string()
    }
}

/// A easy to use test case collection struct that also provide functions for 
/// simple test case creation.
pub struct CaseGroup<T, G, P> {
    cases: Vec<Case<T, G, P>>,
    count: i32,
}

impl<T, G, P> CaseGroup<T, G, P> {

    /// Create a new CaseGroup instance.
    pub fn new() -> CaseGroup<T, G, P> {
        CaseGroup {
            cases: vec![],
            count: 0,
        }
    }

    /// Add existing test case instance to the collection.
    /// 
    /// Note: an additional `index` value will be set by calling this method.
    pub fn add(&mut self, mut case: Case<T, G, P>) {
        self.count += 1;
        case.index = self.count;
        self.cases.push(case);
    }

    /// Get all test cases within current test case collection.
    pub fn all(self) -> Vec<Case<T, G, P>> {
        self.cases
    }
}

/// Generate `create` & `create_param` implementation for different types
///
/// # Note
/// The `<String, G, P>` generics combination has been implemented individually,
/// so you probably should not implement on your own.
#[macro_export]
macro_rules! codegen_case_create_impl {
    ($t:ty, $g:ty, $p:ty) => {
        /// Implement two handy methods on CaseGroup struct.
        impl CaseGroup<$t, $g, $p> {
            /// Create a new test case (no input parameters) matching selected
            /// generic types.
            pub fn create(&mut self, ipt: $t, exp: Vec<$g>) {
                self.add(Case::new(ipt, exp));
            }

            /// Create a new test case (with input parameters) matching
            /// selected generic types.
            pub fn create_param(&mut self, ipt: $t, exp: Vec<$g>, params: Vec<$p>) {
                self.add(Case::new_params(ipt, params, exp));
            }
        }
    };
}

/// Implement two handy methods on CaseGroup<String, G, P> struct.
impl<G, P> CaseGroup<String, G, P> {
    /// Create a new test case (mo input parameters) matching
    /// &str and other generic types.
    /// 
    /// # Argument
    /// * `ipt` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    pub fn create(&mut self, ipt: &str, exp: Vec<G>)
    where
        P: PartialEq + Display,
        G: PartialEq + Display,
    {
        self.add(Case::new(ipt.to_string(), exp));
    }

    /// Create a new test case (with input parameters) matching
    /// &str and other generic types.
    /// 
    /// # Argument
    /// * `ipt` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    /// * `params` - expected values in `Vec<P>` form.
    pub fn create_param(&mut self, ipt: &str, exp: Vec<G>, params: Vec<P>)
    where
        G: PartialEq + Display,
        P: PartialEq + Display,
    {
        self.add(Case::new_params(ipt.to_string(), params, exp));
    }
}


codegen_case_create_impl!(i32, i32, i32);
