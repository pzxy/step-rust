/// 1. 条件(限制)+目的(值)
/// 2. 有向图，但是不能有环，有环就不是动态规划。
/// 3. 大问题拆问题。
/// 4. 最优解，每个条件+目的的限制下都是最优解，每个小问题都是最优解，那么最后的大问题也能拿到最优解
/// 5. 动态规划有的是找最优解(限制条件下)，有的是找某个条件的下的目的(值)
/// 6. 如果最终目的是一个特别的要求，比如要选最大值，或者最小值。那么动态规划公式中一定有min，max这样。

fn main() {
    println!("fib_rec: {}", fib_rec(1));
    println!("fib_dp: {}", fib_dp(1000000007))
}

// 1. fib
// F(0) = 0，F(1) = 1
// F(n) = F(n - 1) + F(n - 2)，其中 n > 1
// 1 1 2 3 5 8 13 21
pub fn fib_rec(n: i32) -> i32 {
    return match n {
        0 => 0,
        1 => 1,
        _ => fib_rec(n - 1) + fib_rec(n - 2),
    };
}

pub fn fib_dp(n: i32) -> i32 {
    if n == 0 { return 0; }
    let n = n as usize;
    let mut opt = vec![0; n+1];
    opt[0] = 0;
    opt[1] = 1;
    for i in 2..=n {
        opt[i] = opt[i - 1] + opt[i - 2];
        opt[i] %= 1000000007;
    }
    return opt[n];
}



