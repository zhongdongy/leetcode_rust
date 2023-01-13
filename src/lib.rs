#[cfg(test)]
mod questions;

#[test]
fn test_q000_005_longest_palindrome() {
    use questions::q000_005::longest_palindrome;
    let mut res;
    res = longest_palindrome(String::from("a"));
    assert!(res == String::from("a"));
    res = longest_palindrome(String::from("abcba"));
    assert_eq!(res, String::from("abcba"));
    res = longest_palindrome(String::from("babac"));
    assert!(res == String::from("bab") || res == String::from("aba"));
    res = longest_palindrome(String::from("bbbbb"));
    assert!(res == String::from("bbbbb"));
    res = longest_palindrome(String::from("bccacc"));
    assert!(res == String::from("ccacc"));
    res = longest_palindrome(String::from("cc"));
    assert!(res == String::from("cc"));
    res = longest_palindrome(String::from("cdd"));
    assert!(res == String::from("dd"));
    res = longest_palindrome(String::from("adddddcbc"));
    assert!(res == String::from("ddddd"));
    res = longest_palindrome(String::from("adddcbc"));
    assert!(res == String::from("ddd") || res == String::from("cbc"));
}