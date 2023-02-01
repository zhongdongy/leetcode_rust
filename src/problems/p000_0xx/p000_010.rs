//! # Description
//!
//! Given an input string `s` and a pattern `p`, implement regular expression
//! matching with support for '`.`' and '`*`' where:
//!
//! - '`.`' Matches any single character.​​​​
//! - '`*`' Matches zero or more of the preceding element.
//! The matching should cover the **entire** input string (not partial).
//!
//! Example 1:
//!
//! ```plain
//! Input: s = "aa", p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//! ```
//!
//! Example 2:
//!
//! ```plain
//! Input: s = "aa", p = "a*"
//! Output: true
//! Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
//! ```
//!
//! Example 3:
//!
//! ```plain
//! Input: s = "ab", p = ".*"
//! Output: true
//! Explanation: ".*" means "zero or more (*) of any character (.)".
//! ```
//!
//! Constraints:
//!
//! - `1 $\leqslant$ s.length $\leqslant$ 20`
//! - `1 $\leqslant$ p.length $\leqslant$ 30`
//! - `s` contains only lowercase English letters.
//! - `p` contains only lowercase English letters, '`.`', and '`*`'.
//! - It is guaranteed for each appearance of the character '`*`', there will be a previous valid character to match.

////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

/// Simplified regular expression match algorithm
///
/// # Arguments
/// * `s` - input string
/// * `p` - pattern
pub fn is_match(s: String, p: String) -> bool {
    alg_1(s, p)
    // TODO: Try to implement with another more effecient approach in the future.
    // alg_3_from_leetcode(s, p) // This approach comes from LeetCode.
}

/// Node struct representing each match pattern segment.
#[derive(Debug)]
struct MatchRuleNode {
    allow_empty: bool,  // Symbol "*"
    allow_repeat: bool, // Symbol "*"
    allow_any: bool,    // Symbol "."
    u8: u8,
}

/// A struct holding parsed pattern.
///
/// Use the `from` method to create new instance and do first parsing.
/// Then call `match_next` with the whole input and parsed nodes to run matching.
#[derive(Debug)]
struct MatchTree {
    labeled_results: HashMap<(usize, usize), bool>,
}
impl MatchTree {
    /// Match next character (English letters only)
    ///
    /// #TODO: document this function
    ///
    /// # Arguments
    /// * `s` - input string of each step in bytes form
    /// * `nodes` - pattern nodes left to match
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

/// Create a new MatchTree instance.
///
/// #TODO: document this function
///
/// # Arguments
/// * `p` - pattern
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

/// Using struct to parse patterns and match input string.
///
/// # Arguments
/// * `s` - input string
/// * `p` - pattern
#[allow(unused)]
fn alg_1(s: String, p: String) -> bool {
    let mut nodes = parse_pattern(p.as_str());
    let mut tree = MatchTree {
        labeled_results: HashMap::new(),
    };

    tree.match_next(&s.as_bytes(), nodes.as_slice())
}

/// A function based variant of algorithm 1.
///
/// # Arguments
/// * `s` - input string
/// * `p` - pattern
#[deprecated(
    since = "0.2.1",
    note = "This function will cause \"Time Limit Exceeded\" error"
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
