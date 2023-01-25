# LeetCode Solutions in Rust
![CI Status](https://github.com/zhongdongy/leetcode_rust/actions/workflows/solution_test.yml/badge.svg)

## Test

Solution tests are located in `test/problems` directory, and grouped by its 
problem numbers. Each problem test group (e.g. `p000_0xxx.rs`) corresponds to a
test case module directory (e.g. `cases/c000_0xx/`). Each problem test has its
own case definitions, to use the test cases, just import and call `use_case()` 
function.

### Test all solutions

To run all solution tests, simply run:

```bash
cargo test --test solutions
```

### Run documentation tests

```bash
cargo test --doc
```

### Run unit tests

```bash
cargo test --lib
```

### Run unit tests and all solutions together

```bash
cargo test --tests
```

### Run tests for specific problem

Suppose you would like to test solution for problem #5, run:

```bash
cargo test --test solutions p000_005
```