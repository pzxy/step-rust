//Instructions
// Given a string of digits, output all the contiguous substrings of length n in that string in the order that they appear.
//
// For example, the string "49142" has the following 3-digit series:
//
// "491"
// "914"
// "142"
// And the following 4-digit series:
//
// "4914"
// "9142"
// And if you ask for a 6-digit series from a 5-digit string, you deserve whatever you get.
//
// Note that these series are only required to occupy adjacent positions in the input; the digits need not be numerically consecutive.



//指示
// 给定一串数字，n按出现的顺序输出该字符串中所有长度的连续子串。
//
// 例如，字符串“49142”具有以下 3 位系列：
//
// “491”
// “914”
// “142”
// 以及以下 4 位系列：
//
// “4914”
// “9142”
// 而且，如果您从 5 位字符串中要求 6 位系列，那么您应得的。
//
// 注意，这些系列只需要在输入中占据相邻位置 即可；数字不必在数字上是连续的。
pub fn series2(digits: &str, len: usize) -> Vec<String> {
    let src: Vec<char> = digits.chars().collect();
    let mut ret = vec![];
    if len > src.len() {
        return ret;
    }
    for v in 0..=src.len() - len {
        let s: String = src[v..len + v].iter().collect();
        ret.push(s)
    }
    ret
}
// 高分
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()
}

#[test]
fn test_with_zero_length() {
    let expected = vec!["".to_string(); 6];
    assert_eq!(series("92017", 0), expected);
}

#[test]
#[ignore]
fn test_with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}

#[test]
#[ignore]
fn test_with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}

#[test]
#[ignore]
fn test_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}

#[test]
#[ignore]
fn test_way_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 42), expected);
}
