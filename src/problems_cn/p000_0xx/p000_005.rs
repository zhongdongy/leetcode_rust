/// 最长回文子串
///
/// 可能出现同一个字符串有多个最长回文子串的现象
/// 参照 `tests/cases_cn/c000_0xx/c000_005.rs` 了解测试用例
///
/// ### Arguments
/// * `s` - 待搜索的原始字符串.
///
/// ```
/// use leetcode_rust::problems_cn::p000_0xx::p000_005::longest_palindrome;
/// let mut result_value = longest_palindrome(String::from("abbab"));
/// assert_eq!(result_value, String::from("abba"));
/// ```
pub fn longest_palindrome(s: String) -> String {
  by_array_index(&s).to_string()
}

#[cfg(test)]
#[test]
fn test_longest_palindrome() {
  assert!(longest_palindrome(String::from("abbbabbbac")) == String::from("abbbabbba"));
}

#[allow(unused_assignments)]
fn by_array_index(s: &str) -> &str {
  let b = s.as_bytes();
  if b.len() == 1 {
      return s;
  }

  let mut cur_longest_start_idx = 0;
  let mut cur_longest_end_idx = 0;
  let mut ite = 1;
  let mut cur_start_idx = 0;
  let mut cur_end_idx = 0;
  let mut should_repeat = false;
  while ite <= b.len() - 1 || should_repeat {
      cur_start_idx = ite - 1;
      cur_end_idx = ite;
      if should_repeat {
          if ite < b.len() - 1 {
              cur_end_idx = ite + 1;
          }
          ite += 1;
      }
      should_repeat = !should_repeat;
      while cur_start_idx > 0 && cur_end_idx < b.len() - 1 && b[cur_end_idx] == b[cur_start_idx] {
          cur_start_idx -= 1;
          cur_end_idx += 1;
      }
      if b[cur_end_idx] != b[cur_start_idx]
          && cur_end_idx - cur_start_idx > 2
          && b[cur_end_idx - 1] == b[cur_start_idx + 1]
      {
          cur_end_idx -= 1;
          cur_start_idx += 1;
      } else if b[cur_end_idx] != b[cur_start_idx] {
          continue;
      }
      if cur_end_idx - cur_start_idx > cur_longest_end_idx - cur_longest_start_idx {
          cur_longest_end_idx = cur_end_idx;
          cur_longest_start_idx = cur_start_idx;
      }
  }
  &s[cur_longest_start_idx..(cur_longest_end_idx + 1)]
}

#[cfg(test)]
#[test]
fn test_by_array_index() {
  assert!(by_array_index("QQ") == String::from("QQ"));
  assert!(by_array_index("QAQ") == String::from("QAQ"));
}
