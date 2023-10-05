fn main() {
    let n = 3;
    let i = climb_stairs(n);
    println!("{}",i);
}

//假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
//
//
//
// 示例 1：
//
// 输入：n = 2
// 输出：2
// 解释：有两种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶
// 2. 2 阶
// 示例 2：
//
// 输入：n = 3
// 输出：3
// 解释：有三种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶 + 1 阶
// 2. 1 阶 + 2 阶
// 3. 2 阶 + 1 阶
pub fn climb_stairs(n: i32) -> i32 {
    let n =  n as usize;
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut opt = vec![0; n + 1];
    opt[1] = 1;
    opt[2] = 2;
    for i in 3..=n  {
        opt[i] = opt[i-1] + opt[i-2]
    }
    return opt[opt.len() - 1];
}