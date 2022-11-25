// Instructions
// Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.
//
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
//
// The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385.
//
// Hence the difference between the square of the sum of the first ten natural numbers and the sum of the squares of the first ten natural numbers is 3025 - 385 = 2640.
//
// You are not expected to discover an efficient solution to this yourself from first principles; research is allowed, indeed, encouraged. Finding the best algorithm for the problem is a key skill in software engineering.

// 求和的平方与前 N 个自然数的平方和之间的差。
//
// 前十个自然数之和的平方是 (1 + 2 + ... + 10)² = 55² = 3025。
//
// 前十个自然数的平方和为 1² + 2² + ... + 10² = 385。
//
// 因此，前十个自然数之和的平方和前十个自然数的平方和之差为 3025 - 385 = 2640。
//
// 您不会从基本原理中自己找到有效的解决方案；研究是允许的，实际上是鼓励的。为问题找到最佳算法是软件工程的一项关键技能。
fn main(){}
pub fn square_of_sum(n: u32) -> u32 {
    //   n * (n + 1) * (2 * n + 1) / 6 ,等差序列
    (0..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    //    (n * (n + 1) / 2).pow(2)
    (0..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}


#[test]
fn test_square_of_sum_1() {
    assert_eq!(1, square_of_sum(1));
}

#[test]
#[ignore]
fn test_square_of_sum_5() {
    assert_eq!(225, square_of_sum(5));
}

#[test]
#[ignore]
fn test_square_of_sum_100() {
    assert_eq!(25_502_500, square_of_sum(100));
}

#[test]
#[ignore]
fn test_sum_of_squares_1() {
    assert_eq!(1, sum_of_squares(1));
}

#[test]
#[ignore]
fn test_sum_of_squares_5() {
    assert_eq!(55, sum_of_squares(5));
}

#[test]
#[ignore]
fn test_sum_of_squares_100() {
    assert_eq!(338_350, sum_of_squares(100));
}

#[test]
#[ignore]
fn test_difference_1() {
    assert_eq!(0, difference(1));
}

#[test]
#[ignore]
fn test_difference_5() {
    assert_eq!(170, difference(5));
}

#[test]
#[ignore]
fn test_difference_100() {
    assert_eq!(25_164_150, difference(100));
}