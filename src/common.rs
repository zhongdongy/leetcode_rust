pub struct Case<T, G> {
    pub input: T,
    pub params: Vec<G>,
    pub values: Vec<T>,
}
use std::fmt;

impl<T, G> Case<T, G>
where
    T: PartialEq + fmt::Display,
{
    pub fn new(input: T, values: Vec<T>) -> Case<T, G> {
        Case {
            input: input,
            params: vec![],
            values: values,
        }
    }
    pub fn new_params(input: T, params: Vec<G>, values: Vec<T>) -> Case<T, G> {
        Case {
            input: input,
            params: params,
            values: values,
        }
    }
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
            assert!(false, "Result `{}` != expectation `{}`", &result, self.values[0]);
        } else {
            assert!(false, "Result `{}` doesn't match any expectations", &result);
        }
    }
}
