//! # 问题描述
//!
//! 给你一个字符串 `s` 和一个字符规律 `p`，请你来实现一个支持 '`.`' 和 '`*`' 的正则表达式匹配。
//! 
//! - '`.`' 匹配任意单个字符
//! - '`*`' 匹配零个或多个前面的那一个元素
//! - 所谓匹配，是要涵盖**整个**字符串 `s` 的，而不是部分字符串。
//! 
//!  
//! 示例 1：
//! 
//! ```plain
//! 输入：s = "aa", p = "a"
//! 输出：false
//! 解释："a" 无法匹配 "aa" 整个字符串。
//! ```
//! 
//! 示例 2:
//! 
//! ```plain
//! 输入：s = "aa", p = "a*"
//! 输出：true
//! 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
//! ```
//! 
//! 示例 3：
//! 
//! ```plain
//! 输入：s = "ab", p = ".*"
//! 输出：true
//! 解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
//! ```
//! 
//! 提示：
//! 
//! - `1 $\leqslant$ s.length $\leqslant$ 20`
//! - `1 $\leqslant$ p.length $\leqslant$ 30`
//! - `s` 只包含从 `a-z` 的小写字母。
//! - `p` 只包含从 `a-z` 的小写字母，以及字符 `.` 和 `*`。
//! - 保证每次出现字符 `*` 时，前面都匹配到有效的字符
//! 
//! 来源：<https://leetcode.cn/problems/regular-expression-matching>

////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

/// 简化版的正则表达式匹配算法
///
/// # 参数
/// * `s` - 输入字符串
/// * `p` - 模式
pub fn is_match(s: String, p: String) -> bool {
    alg_1(s, p)
    // TODO: Try to implement with another more effecient approach in the future.
    // alg_3_from_leetcode(s, p) // This approach comes from LeetCode.
}

/// 节点 struct，用于描述模式中的每个部分
#[derive(Debug)]
struct MatchRuleNode {
    allow_empty: bool,  // 符号 "*"
    allow_repeat: bool, // 符号 "*"
    allow_any: bool,    // 符号 "."
    u8: u8,
}

/// 用于记录已经解析、判定过的模式的 struct
///
/// 传入完整待匹配字符串和解析后的模式来调用 `match_next` 方法
#[derive(Debug)]
struct MatchTree {
    labeled_results: HashMap<(usize, usize), bool>,
}
impl MatchTree {
    /// 匹配剩下的部分（仅匹配英文字母）
    ///
    /// #TODO: 为这个函数写注释
    ///
    /// # 参数
    /// * `s` - 字节数组形式的待匹配部分
    /// * `nodes` - 等待匹配的模式
    fn match_next(&mut self, s: &[u8], nodes: &[MatchRuleNode]) -> bool {
        macro_rules! label_pair {
            ($s:expr, $p:expr) => {{
                if !self.labeled_results.contains_key(&($s, $p)) {
                    self.labeled_results.insert(($s, $p), false);
                }
                false
            }};
        }

        macro_rules! match_if_not_labeled {
            ($s:expr, $n:expr) => {
                if self.labeled_results.contains_key(&($s.len(), $n.len())) {
                    *self.labeled_results.get(&($s.len(), $n.len())).unwrap()
                } else {
                    self.match_next($s, $n)
                }
            };
        }

        if nodes.len() == 0 && s.len() > 0 {
            label_pair!(s.len(), nodes.len())
        } else if s.len() == 0 && nodes.len() > 0 {
            for n in nodes {
                if !n.allow_empty {
                    return label_pair!(s.len(), nodes.len());
                }
            }
            true
        } else if s.len() == 0 && nodes.len() == 0 {
            true
        } else {
            if nodes.len() == 1 && !nodes[0].allow_any {
                // Try fast exit
                for ch in s {
                    if *ch != nodes[0].u8 {
                        return label_pair!(s.len(), nodes.len());
                    }
                }
            }

            if nodes[0].allow_any {
                if nodes[0].allow_repeat {
                    match_if_not_labeled!(&s[1..], &nodes[1..])
                        || match_if_not_labeled!(s, &nodes[1..])
                        || match_if_not_labeled!(&s[1..], nodes)
                } else {
                    match_if_not_labeled!(&s[1..], &nodes[1..])
                }
            } else if nodes[0].u8 == s[0] {
                if nodes[0].allow_repeat {
                    match_if_not_labeled!(&s[1..], &nodes[1..])
                        || match_if_not_labeled!(s, &nodes[1..])
                        || match_if_not_labeled!(&s[1..], nodes)
                } else {
                    match_if_not_labeled!(&s[1..], &nodes[1..])
                }
            } else if nodes[0].allow_empty {
                match_if_not_labeled!(s, &nodes[1..])
            } else {
                label_pair!(s.len(), nodes.len())
            }
        }
    }
}

/// 解析并构建 MatchRuleNode 数组
///
/// #TODO: 为函数写注释
///
/// # 参数
/// * `p` - 待解析的模式
fn parse_pattern(p: &str) -> Vec<MatchRuleNode> {
    let mut nodes = vec![];

    let mut idx = 0;
    let pbytes = p.as_bytes();
    loop {
        if idx == pbytes.len() {
            break;
        }
        let mut temp_node = MatchRuleNode {
            allow_empty: true,
            allow_repeat: true,
            allow_any: pbytes[idx] == 46,
            u8: pbytes[idx],
        };
        if pbytes.len() - 1 > idx && pbytes[idx + 1] == 42 {
            idx += 2;
        } else {
            temp_node.allow_empty = false;
            temp_node.allow_repeat = false;
            idx += 1;
        }
        if nodes.len() > 0 {
            let last: &MatchRuleNode = nodes.last().unwrap();
            if last.u8 == temp_node.u8
                && temp_node.allow_repeat == true
                && last.allow_any == temp_node.allow_any
                && last.allow_repeat == temp_node.allow_repeat
                && last.allow_empty == temp_node.allow_empty
            {
                // Skip this
                continue;
            }
        }
        nodes.push(temp_node);
    }

    nodes
}

/// 使用模式匹配构建的 struct 来匹配传入字符串
///
/// # 参数
/// * `s` - 传入字符串
/// * `p` - 模式字符串
#[allow(unused)]
fn alg_1(s: String, p: String) -> bool {
    let mut nodes = parse_pattern(p.as_str());
    let mut tree = MatchTree {
        labeled_results: HashMap::new(),
    };

    tree.match_next(&s.as_bytes(), nodes.as_slice())
}

/// 使用模式匹配构建的元组来匹配传入字符串
///
/// # 参数
/// * `s` - 传入字符串
/// * `p` - 模式字符串
#[deprecated(
    since = "0.2.1",
    note = "此方法会导致 \"Time Limit Exceeded\" 错误"
)]
#[allow(unused)]
fn alg_2(s: String, p: String) -> bool {
    let mut match_tree: Vec<(bool, bool, bool, u8)> = vec![];

    let mut idx = 0;
    let pbytes = p.as_bytes();
    loop {
        if idx == pbytes.len() {
            break;
        }
        let mut temp_node = (true, true, pbytes[idx] == 46, pbytes[idx]);
        if pbytes.len() - 1 > idx && pbytes[idx + 1] == 42 {
            idx += 2;
        } else {
            temp_node.0 = false;
            temp_node.1 = false;
            idx += 1;
        }
        if match_tree.len() > 1 {
            let last = match_tree.last().unwrap();
            if last.3 == temp_node.3
                && temp_node.1 == true
                && last.2 == temp_node.2
                && last.1 == temp_node.1
                && last.0 == temp_node.0
            {
                // Skip this
                continue;
            }
        }
        match_tree.push(temp_node);
    }

    fn recur(s: &[u8], nodes: &[(bool, bool, bool, u8)]) -> bool {
        if nodes.len() == 0 && s.len() > 0 {
            false
        } else if s.len() == 0 && nodes.len() > 0 {
            for n in nodes {
                if !n.0 {
                    return false;
                }
            }
            true
        } else if s.len() == 0 && nodes.len() == 0 {
            true
        } else {
            if nodes[0].2 {
                if nodes[0].1 {
                    recur(&s[1..], nodes) || recur(&s[1..], &nodes[1..]) || recur(s, &nodes[1..])
                } else {
                    recur(&s[1..], &nodes[1..])
                }
            } else if nodes[0].3 == s[0] {
                if nodes[0].1 {
                    recur(&s[1..], nodes) || recur(&s[1..], &nodes[1..]) || recur(s, &nodes[1..])
                } else {
                    recur(&s[1..], &nodes[1..])
                }
            } else if nodes[0].0 {
                recur(s, &nodes[1..])
            } else {
                false
            }
        }
    }

    recur(s.as_bytes(), &match_tree)
}

////////////////////////////////////////////////////////////////////////////////

struct State {
    ch: u8,
    wildcard: bool,
}

/// A demo submission from LeetCode
///
/// Source: <https://leetcode.com/problems/regular-expression-matching/submissions/889316368/>
///
/// # Arguments
/// * `s` - input string
/// * `p` - pattern to match
#[allow(unused)]
fn alg_3_from_leetcode(s: String, p: String) -> bool {
    let states: Vec<State> = p
        .bytes()
        .enumerate()
        .filter_map(|(i, ch)| {
            if ch == b'*' {
                None
            } else {
                Some(State {
                    ch,
                    wildcard: p.as_bytes().get(i + 1).copied() == Some(b'*'),
                })
            }
        })
        .chain([State {
            ch: 0,
            wildcard: false,
        }])
        .collect();
    let state_count = states.len();

    let mut cur_states = vec![false; state_count];
    let add_state = |dest: &mut Vec<bool>, mut i: usize| {
        while !dest[i] {
            dest[i] = true;
            if states[i].wildcard {
                i += 1;
            } else {
                break;
            }
        }
    };

    // Initialization
    add_state(&mut cur_states, 0);

    // Iterate through s.
    for ch in s.into_bytes() {
        if cur_states.is_empty() {
            break;
        }
        let mut new_states = vec![false; state_count];
        for i in 0..state_count {
            if cur_states[i] && (states[i].ch == b'.' || states[i].ch == ch) {
                if states[i].wildcard {
                    add_state(&mut new_states, i);
                } else {
                    add_state(&mut new_states, i + 1);
                }
            }
        }
        cur_states = new_states;
    }

    cur_states.last().copied().unwrap_or(false)
}
