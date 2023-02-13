//! # LeetCode Solutions in Rust
//! 
//! ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/zhongdongy/leetcode_rust/solution_test.yml?label=LeetCode%20Solution%20Tests&logo=github&style=plastic)
//!
//! [阅读中文内容](#rust-语言下的力扣解法非官方)
//! 
//! Rust documentations see [https://leetcode-rust.zhongdongy.com](https://leetcode-rust.zhongdongy.com) 
//! or [https://leetcode-rust.pages.dev/](https://leetcode-rust.pages.dev/). 
//! Additional benchmarks available on 
//! [https://leetcode-rust.zhongdongy.com/criterion/report/](https://leetcode-rust.zhongdongy.com/criterion/report/index.html)
//!
//! Note: all problems descriptions collected from 
//! [LeetCode website](https://leetcode.com/), [力扣](https://leetcode.cn/) and 
//! all related credits go to LeetCode, 力扣 and its communities. Authors of 
//! this repository CANNOT and WILL NOT guarantee the correctness of the 
//! problem descriptions and solutions.
//!
//! ## Documentation
//!
//! Run `cargo doc` and find the documentation inside `target/doc/leetcode_rust/` directory.
//!
//! ### Build documentations
//!
//! This command will empty the `docs/` directory and place newly generated docs
//! there. The documentations are deployed to Cloudflare Pages. Due to build
//! environment limitations, the docs must be built before pushing to GitHub (so
//! Cloudflare directly fetches the contents and build on Pages).
//!
//! ```bash
//! ./docsgen.sh
//! ```
//!
//! ## Test
//!
//! Solution tests are located in `tests/problems` directory, and grouped by its
//! problem numbers. Each problem test group (e.g. `p000_0xxx.rs`) corresponds to a
//! test case module directory (e.g. `cases/c000_0xx/`). Each problem test has its
//! own case definitions, to use the test cases, just import and call `use_case()`
//! function.
//!
//! ### Test all solutions
//!
//! To run all solution tests, simply run:
//!
//! ```bash
//! cargo test --test solutions # LeetCode problems
//! cargo test --test solutions_cn # 力扣题库
//! ```
//!
//! ### Run documentation tests
//!
//! ```bash
//! cargo test --doc
//! ```
//!
//! ### Run unit tests
//!
//! ```bash
//! cargo test --lib
//! ```
//!
//! ### Run unit tests and all solutions together
//!
//! ```bash
//! cargo test --tests
//! ```
//!
//! ### Run tests for specific problem
//!
//! Suppose you would like to test solution for problem #5, run:
//!
//! ```bash
//! cargo test --test solutions p000_005
//! ```
//! 
//! ---
//! 
//! # Rust 语言下的力扣解法（非官方）
//! 
//! ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/zhongdongy/leetcode_rust/solution_test_cn.yml?label=力扣解法验证&logo=github&style=plastic)
//! 
//! [For English readers](#leetcode-solutions-in-rust)
//! 
//! 关于 Crate 提供的各个解法的 Rust 文档，请参阅 [https://leetcode-rust.dongs.xyz/](https://leetcode-rust.dongs.xyz/)。
//! 部分 Benchmark 记录 (由 Criterion.rs 提供) 请参阅 
//! [https://leetcode-rust.dongs.xyz/criterion/report/](https://leetcode-rust.dongs.xyz/criterion/report/index.html)
//! 
//! 请注意，所有的题目来源于 [LeetCode 网站](https://leetcode.com/)和
//! [力扣](https://leetcode.cn/)。所有题目的著作权属于上述两个平台及相应的社区。
//! 作者**不能**也**不会**保证题目描述、解法的正确性。
//!
//! ## 文档
//!
//! 在项目根目录执行 `cargo doc` 命令，然后访问 `target/doc/leetcode_rust/` 目录可以
//! 找到生成的最新文档。
//!
//! ### 构建文档
//!
//! 下述命令将会清空您本地项目目录下的 `docs/` 文件夹，然后将新生成的内容放入其中。由于此文档
//! 部署在 Cloudflare Pages 服务上，而该服务暂不支持使用 Cargo 构建，所以必须在每次推送
//! 最新更改到 GitHub 之前执行本地文档构建。
//!
//! ```bash
//! ./docsgen.sh
//! ```
//!
//! ## 解法验证
//!
//! 所有的解法验证程序都位于 `tests/problems_cn` 目录下，并按照题目编号进行分组。
//! 每个问题组（如 `p000_0xx.rs` 代表编号 000 到 099 的题目）对应一个单独的测试用例目录
//! （如 `cases/c000_0xx/`）。每个问题的验证程序都提供了国际版、国内版两组测试用例，但其中的
//! 大部分都相同样的。要使用某个问题的测试用例，只需要将其引入，然后调用公有的 `use_case()` 
//! 函数，它的返回值就是测试用例列表。
//!
//! ### 验证所有的问题的解法
//!
//! 执行下面的命令来执行所有问题的解法:
//!
//! ```bash
//! cargo test --test solutions # LeetCode 国际版题库
//! cargo test --test solutions_cn # 力扣题库
//! ```
//!
//! ### 执行文档测试
//!
//! ```bash
//! cargo test --doc
//! ```
//!
//! ### 执行单元测试
//!
//! ```bash
//! cargo test --lib
//! ```
//!
//! ### 同时执行单元测试和验证解法
//!
//! ```bash
//! cargo test --tests
//! ```
//!
//! ### 单独验证某个问题的解法
//!
//! 假设您想要验证问题 #5 的解，那么可以执行下面的命令:
//!
//! ```bash
//! cargo test --test solutions p000_005
//! ```
 
////////////////////////////////////////////////////////////////////////////////

pub mod macros;
pub mod models;
pub mod common;
pub mod problems;
pub mod problems_cn;
pub mod cases;
pub mod cases_cn;