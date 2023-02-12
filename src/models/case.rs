use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestCaseGroup<T, G> {
    pub cases: Vec<TestCase<T, G>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCase<T, G> {
    pub input: T,
    pub expectation: G,
}