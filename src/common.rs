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

use crate::macros::codegen_case_create_impl;
use crate::macros::codegen_vector_case_create_impl;
use std::fmt::Debug;

/// Test case wrapper struct
///
/// # Generics
/// * `T` - type of test inputs
/// * `G` - type of expectations, must implement `PartialEq` and `Display` traits
/// * `P` - type of optional parameters
pub struct Case<T, G, P> {
    /// Input values of test case
    pub inputs: Vec<T>,

    /// Optional parameters when executing test case
    pub params: Vec<P>,

    /// Expected values of given input
    pub values: Vec<G>,

    /// Test case index (for labeling)
    index: i32,
}

impl<T, G, P> Case<T, G, P>
where
    T: PartialEq + Debug,
    G: PartialEq + Debug,
    P: PartialEq + Debug,
{
    /// Create new test case with no parameters
    ///
    /// # Arguments
    /// * `input` - test input
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new(input: T, values: Vec<G>) -> Case<T, G, P> {
        Case {
            inputs: vec![input],
            params: vec![],
            values: values,
            index: 0,
        }
    }

    /// Create new test case with no parameters but multiple inputs
    ///
    /// # Arguments
    /// * `inputs` - test input, accept single- or multi-value vector
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new_multi(inputs: Vec<T>, values: Vec<G>) -> Case<T, G, P> {
        Case {
            inputs: inputs,
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
            inputs: vec![input],
            params: params,
            values: values,
            index: 0,
        }
    }

    /// Create new test case with parameters and multi input
    ///
    /// # Arguments
    /// * `inputs` - test input, accept single- or multi-value vector
    /// * `params` - test parameters that vary among different cases
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new_params_multi(inputs: Vec<T>, params: Vec<P>, values: Vec<G>) -> Case<T, G, P> {
        Case {
            inputs: inputs,
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
            if self.inputs.len() == 1 {
                assert!(
                    false,
                    "[#{}] INPUT=`{:?}`, OUTPUT=`{:?}`, EXPECTATION=`{:?}`",
                    self.index, &self.inputs[0], &result, self.values[0]
                );
            } else {
                assert!(
                    false,
                    "[#{}] INPUT=`[{:?}]`, OUTPUT=`{:?}`, EXPECTATION=`{:?}`",
                    self.index, self.inputs, &result, self.values[0]
                );
            }
        } else {
            assert!(
                false,
                "Result `{:?}` doesn't match any expectations",
                &result
            );
        }
    }

    /// Returns the index value in String form.
    pub fn label(&self) -> String {
        self.index.to_string()
    }

    /// Returns the first element of inputs
    pub fn input(&self) -> &T {
        &self.inputs[0]
    }
}

/// A easy to use test case collection struct that also provide functions for
/// simple test case creation.
pub struct CaseGroup<T, G, P> {
    /// A vector containing all test cases.
    cases: Vec<Case<T, G, P>>,

    // Number of test cases included. Used when labeling each case
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

/// Implement two handy methods on CaseGroup<String, G, P> struct.
impl<G, P> CaseGroup<String, G, P>
where
    P: PartialEq + Debug,
    G: PartialEq + Debug,
{
    /// Create a new test case (no input parameters) matching
    /// &str and other generic types.
    ///
    /// # Argument
    /// * `ipt` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    pub fn create(&mut self, ipt: &str, exp: Vec<G>) {
        self.add(Case::new(ipt.to_string(), exp));
    }

    /// Create a new test case (with input parameters) matching
    /// &str and other generic types.
    ///
    /// # Argument
    /// * `ipt` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    /// * `params` - expected values in `Vec<P>` form.
    pub fn create_param(&mut self, ipt: &str, exp: Vec<G>, params: Vec<P>) {
        self.add(Case::new_params(ipt.to_string(), params, exp));
    }

    /// Create a new test case (no input parameters but multiple inputs)
    /// matching &str and other generic types.
    ///
    /// # Argument
    /// * `ipts` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    pub fn create_multi(&mut self, ipts: Vec<&str>, exp: Vec<G>) {
        self.add(Case::new_multi(
            ipts.iter().map(|x| x.to_string()).collect(),
            exp,
        ));
    }

    /// Create a new test case (with input parameters and multiple inputs)
    /// matching &str and other generic types.
    ///
    /// # Argument
    /// * `ipts` - this argument is set to `&str` to simplify method calls.
    /// * `exp` - expected values in `Vec<G>` form.
    /// * `params` - expected values in `Vec<P>` form.
    pub fn create_param_multi(&mut self, ipts: Vec<&str>, exp: Vec<G>, params: Vec<P>) {
        self.add(Case::new_params_multi(
            ipts.iter().map(|x| x.to_string()).collect(),
            params,
            exp,
        ));
    }
}

codegen_case_create_impl!(i32, i32, i32);
codegen_case_create_impl!(i32, String, i32);
codegen_case_create_impl!(i32, bool, i32);
codegen_case_create_impl!(Vec<i32>, i32, i32);

pub struct VectorCase<T, G, P> {
    /// Input values of test case
    pub inputs: Vec<T>,

    /// Optional parameters when executing test case
    pub params: Vec<P>,

    /// Expected values of given input
    pub values: Vec<G>,

    /// Test case index (for labeling)
    index: i32,
}

impl<T, G, P> VectorCase<T, G, P>
where
    T: PartialEq,
    G: PartialEq,
    P: PartialEq,
{
    /// Create new test case with no parameters
    ///
    /// # Arguments
    /// * `input` - test input
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new(input: Vec<T>, values: Vec<G>) -> VectorCase<T, G, P> {
        VectorCase {
            inputs: input,
            params: vec![],
            values: values,
            index: 0,
        }
    }

    /// Create new test case with no parameters but multiple inputs
    ///
    /// # Arguments
    /// * `inputs` - test input, accept single- or multi-value vector
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new_multi(inputs: Vec<T>, values: Vec<G>) -> VectorCase<T, G, P> {
        VectorCase {
            inputs: inputs,
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
    pub fn new_params(input: Vec<T>, params: Vec<P>, values: Vec<G>) -> VectorCase<T, G, P> {
        VectorCase {
            inputs: input,
            params: params,
            values: values,
            index: 0,
        }
    }

    /// Create new test case with parameters and multi input
    ///
    /// # Arguments
    /// * `inputs` - test input, accept single- or multi-value vector
    /// * `params` - test parameters that vary among different cases
    /// * `values` - expected values, accept single- or multi-value vector
    pub fn new_params_multi(inputs: Vec<T>, params: Vec<P>, values: Vec<G>) -> VectorCase<T, G, P> {
        VectorCase {
            inputs: inputs,
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
    /// ```rust
    /// use leetcode_rust::common::VectorCase;
    /// let case:VectorCase<String, String, i32> = VectorCase::new(vec![String::from("A")], vec![String::from("A")]);
    /// case.is_valid(String::from("A"))
    /// ```
    pub fn is_valid(&self, result: G)
    where
        G: Debug,
        T: Debug,
    {
        if self.values.len() == 0 {
            assert!(false);
        }
        for val in self.values.iter() {
            if *val == result {
                return assert!(true);
            }
        }

        if self.values.len() == 1 {
            if self.inputs.len() == 1 {
                assert!(
                    false,
                    "[#{}] not match\nInput=`{:?}`\nOutput=`{:?}`\nExpect=`{:?}`",
                    self.index,
                    self.input(),
                    result,
                    self.values[0]
                );
            } else {
                assert!(false, "[#{}]", self.index);
            }
        } else {
            assert!(false, "Result doesn't match any expectations");
        }
    }

    /// Returns the index value in String form.
    pub fn label(&self) -> String {
        self.index.to_string()
    }

    /// Returns the first element of inputs
    pub fn input(&self) -> &T {
        &self.inputs[0]
    }
}

pub struct VectorCaseGroup<T, G, P> {
    /// A vector containing all test cases.
    cases: Vec<VectorCase<T, G, P>>,

    // Number of test cases included. Used when labeling each case
    count: i32,
}

impl<T, G, P> VectorCaseGroup<T, G, P> {
    /// Create a new VectorCaseGroup instance.
    pub fn new() -> VectorCaseGroup<T, G, P> {
        VectorCaseGroup {
            cases: vec![],
            count: 0,
        }
    }

    /// Add existing test case instance to the collection.
    ///
    /// Note: an additional `index` value will be set by calling this method.
    pub fn add(&mut self, mut case: VectorCase<T, G, P>) {
        self.count += 1;
        case.index = self.count;
        self.cases.push(case);
    }

    /// Get all test cases within current test case collection.
    pub fn all(self) -> Vec<VectorCase<T, G, P>> {
        self.cases
    }
}

codegen_vector_case_create_impl!(Vec<i32>, Vec<Vec<i32>>, i32);
