//写一个函数，输入 n ，求斐波那契（Fibonacci）数列的第 n 项（即 F(N)）。斐波那契数列的定义如下：
//
// F(0) = 0, F(1)= 1
// F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
// 斐波那契数列由 0 和 1 开始，之后的斐波那契数就是由之前的两数相加而得出。
//
// 答案需要取模 1e9+7（1000000007），如计算初始结果为：1000000008，请返回 1。
//
//
// 示例 1：
//
// 输入：n = 2
// 输出：1
// 示例 2：
//
// 输入：n = 5
// 输出：5

//
// 提示：
//
// 0 <= n <= 100
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/fei-bo-na-qi-shu-lie-lcof
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
fn main() {
    let aa = fib22(48);
    println!("{}", aa);
}
pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

// 这种写法好巧妙,比递归好理解多了.
// 使用元组,真正模拟了 斐波那契数列生成过程. 0 1,1 2,2 3,3 5,5 8,8 13
pub fn fib2(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        // (1..n): 迭代(1..n),每次迭代值为 变量x
        // (0,1): 为a的初始值 accum,
        // 开始迭代: accum = f(accum,x)
        _ => {
            (1..n)
                .fold((0, 1), |mut a, x| {
                    (a.1 % 1000000007, (a.0 + a.1) % 1000000007)
                })
                .1
        }
    }
}

pub fn fib2222(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            // 类似 go 里面的 map range{}
            // f 执行以后会将结构再赋值给 ret
            (1..n).fold((0, 1), |mut ret, v| {
                (ret.1 % 1000000007, (ret.0 + ret.1) % 1000000007)
            }).1
        }
    }
}

// 这是不对的,
pub fn fib22(n: i32) -> i32 {
    // fold 会推断是 i32 类型,但是 i32 的话, 值会溢出. 试了下 u128 比较合适.
    let ret = (0..n).fold((0, 1), |acc, _| {
        let b = acc.0 + acc.1;
        println!("b={}", b);
        (acc.1, b)
    });
    (ret.0 % 1000000007) as i32
}

pub fn fib222(n: i32) -> i32 {
    ((0..n)
        .fold((0u128, 1u128), |acc, _| (acc.1, acc.0 + acc.1))
        .0
        % 1000000007) as i32
}

pub fn fib3(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            (1..n)
                .fold((0, 1), |mut a, x| {
                    (a.1 & 1000000006, (a.0 + a.1) & 1000000007)
                })
                .1
        }
    }
}

pub fn fold2() {
    let vec = vec![1, 2, 3, 4, 5];
    let res = vec.iter().fold(0, |acc, x| acc + x);
    eprint!("{}", res);
}
