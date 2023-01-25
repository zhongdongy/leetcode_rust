pub fn longest_palindrome(s: String) -> String {
    by_array_index(&s).to_string()
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
