use std::fmt;

/// Test case wrapper struct
///
/// ### Generics
/// * `T` type of expectations, must implement `PartialEq` and `Display` traits
/// * `G` type of optional parameters
pub struct Case<T, G> {
    /// Input value of test case
    pub input: T,

    /// Optional parameters when executing test case
    pub params: Vec<G>,

    /// Expected values of given input
    pub values: Vec<T>,
}

impl<T, G> Case<T, G>
where
    T: PartialEq + fmt::Display,
{
    /// Create new test case with no parameters
    ///
    /// ### Arguments
    /// * `input` test input
    /// * `values` expected values, accept single- or multi-value vector
    pub fn new(input: T, values: Vec<T>) -> Case<T, G> {
        Case {
            input: input,
            params: vec![],
            values: values,
        }
    }

    /// Create new test case with parameters
    ///
    /// ### Arguments
    /// * `input` test input
    /// * `params` test parameters that vary among different cases
    /// * `values` expected values, accept single- or multi-value vector
    pub fn new_params(input: T, params: Vec<G>, values: Vec<T>) -> Case<T, G> {
        Case {
            input: input,
            params: params,
            values: values,
        }
    }

    /// Check if solution output matches any expectations
    ///
    /// ### Arguments
    /// * `&self` inmutable borrow to object itself
    /// * `result` solution output
    ///
    /// ```
    /// use leetcode_rust::common::Case;
    /// let case:Case<String, i32> = Case::new(String::from("A"), vec![String::from("A")]);
    /// case.is_valid(String::from("A"))
    /// ```
    pub fn is_valid(&self, result: T) {
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
                "Result `{}` != expectation `{}`",
                &result, self.values[0]
            );
        } else {
            assert!(false, "Result `{}` doesn't match any expectations", &result);
        }
    }
}
