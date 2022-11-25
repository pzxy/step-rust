//Instructions
// The Collatz Conjecture or 3x+1 problem can be summarized as follows:
//
// Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.
//
// But sometimes the number grow significantly before it reaches 1. This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64.
//
// Given a number n, return the number of steps required to reach 1.
//指示
// Collatz 猜想或 3x+1 问题可以总结如下：
//
// 取任意正整数 n。如果 n 是偶数，将 n 除以 2 得到 n / 2。如果 n 是奇数，将 n 乘以 3 并加 1 得到 3n + 1。无限重复该过程。该猜想表明，无论您从哪个数字开始，最终总会达到 1。
//
// 但有时数字在达到 1 之前会显着增长。这可能导致整数溢出，并使算法在 u64 中的数字范围内无法解决。
//
// 给定一个数字 n，返回达到 1 所需的步数。
//
// 例子
// 从 n = 12 开始，步骤如下：
//
// 12
// 6
// 3
// 10
// 5
// 16
// 8
// 4
// 2
// 1
// 结果是9个步骤。因此，对于输入 n = 12，返回值为 9。

//[package]
// edition = "2021"
// name = "collatz_conjecture"
// version = "1.2.1"
fn main(){}

pub fn collatz2(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            v if v & 1 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}

// 高分
pub fn collatz(mut n: u64) -> Option<u64> {
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            even if even % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}


#[test]
fn test_1() {
    assert_eq!(Some(0), collatz(1));
}

#[test]
#[ignore]
fn test_16() {
    assert_eq!(Some(4), collatz(16));
}

#[test]
#[ignore]
fn test_12() {
    assert_eq!(Some(9), collatz(12));
}

#[test]
#[ignore]
fn test_1000000() {
    assert_eq!(Some(152), collatz(1_000_000));
}

#[test]
#[ignore]
fn test_0() {
    assert_eq!(None, collatz(0));
}

#[test]
#[ignore]
fn test_110243094271() {
    let val = 110243094271;
    assert_eq!(None, collatz(val));
}

#[test]
#[ignore]
fn test_max_div_3() {
    let max = u64::MAX / 3;
    assert_eq!(None, collatz(max));
}

#[test]
#[ignore]
fn test_max_minus_1() {
    let max = u64::MAX - 1;
    assert_eq!(None, collatz(max));
}
