//给你一个字符串 s，找到 s 中最长的回文子串。
// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
//
// 示例 1：
// 输入：s = "babad"
// 输出："bab"
// 解释："aba" 同样是符合题意的答案
//
// 示例 2：
// 输入：s = "cbbd"
// 输出："bb"
//
// 思路：
// 一判断： 有最优解、增加数组长度解题思路不变、小问题最优解组合成了大问题最优解(abcdedcba)。所以可以使用动态规划。
// 二状态转移方程：
// - 一个确定回文的函数。这个在 例子一数字和 中的体现是求和方法。
// - 求出每个可能性的函数，然后选长度最大的一个。这里一个是选当前坐标的值，
// 有三种情况：
// 1.之前还没选过，判断是aba这种结构;
// 2. 之前选过了，并且和原来相同长度相同；
// 3. 之前选过了，并且判断出是当前坐标的前一个，那么现在一定是xabax的这种；
// 如果不选当前：
// 1. 不符合条件，
// 2. 符合条件，判断没有以前的长度长。

use std::collections::HashMap;

//
fn main() {
    let s = "babad".to_owned();
    let ret = longest_palindrome(s);
    println!("{}", ret);
}

/// 方法一：动态规划：dp[i][j] 表示 s[i]..s[j] 是不是回文
/// 那么 dp[i][j] = (s[i]==s[j]) && dp[i+1][j-1], 即在回文数组外面套壳
/// 填表策略：按长度填表-->0,1,2,...
/// 时空复杂度都为 O(n^2)
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let s:Vec<char> = s.chars().collect();

    let mut dp = vec![vec![true; n]; n];
    let mut res = (0,0);

    for k in 1..n{
        for i in 0..(n-k){
            dp[i][i+k] = (s[i]==s[i+k]) && dp[i+1][i+k-1];
            if dp[i][i+k]{
                res = (i, i+k);
            }
        }
    };

    s[res.0..=res.1].into_iter().collect::<String>()
}


fn is_palindrome(s: &str, l: usize, r: usize) -> bool {
    if l >= r {
        return false;
    }
    let mut l = l;
    let mut r = r;
    while l < r {
        if s.as_bytes()[l] != s.as_bytes()[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    return true;
}