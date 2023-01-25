# LeetCode Solutions in Rust
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/zhongdongy/leetcode_rust/solution_test.yml?label=LeetCode%20Solution%20Tests&logo=github&style=plastic)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/zhongdongy/leetcode_rust/solution_test_cn.yml?label=力扣解法验证&logo=github&style=plastic)

## Test

Solution tests are located in `test/problems` directory, and grouped by its 
problem numbers. Each problem test group (e.g. `p000_0xxx.rs`) corresponds to a
test case module directory (e.g. `cases/c000_0xx/`). Each problem test has its
own case definitions, to use the test cases, just import and call `use_case()` 
function.

### Test all solutions

To run all solution tests, simply run:

```bash
cargo test --test solutions # LeetCode problems
cargo test --test solutions_cn # 力扣题库
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