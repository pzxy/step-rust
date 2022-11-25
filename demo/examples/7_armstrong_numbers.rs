// Instructions
// An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.
//
// For example:
//
// 9 is an Armstrong number, because 9 = 9^1 = 9
// 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
// 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
// Write some code to determine whether a number is an Armstrong number.

//阿姆斯特朗数是一个数字，它是它自己的数字的总和，每个数字都提高到数字的幂。
//
// 例如：
//
// 9 是阿姆斯壮数，因为9 = 9^1 = 9
// 10不是阿姆斯壮数，因为10 != 1^2 + 0^2 = 1
// 153 是阿姆斯壮数，因为：153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154不是阿姆斯壮数，因为：154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
// 编写一些代码来确定一个数字是否是 Armstrong 数字。
fn main(){}
// 好丑的代码
pub fn is_armstrong_number_me(num: u32) -> bool {
    let s = num.to_string();
    let n = s.len() as u32;
    let mut sum = 0u64;
    for c in s.chars() {
        let v: u64 = c.to_string().parse().unwrap();
        let tmp = v.pow(n) as u64;
        if tmp > num as u64 {
            return false;
        }
        sum += tmp;
    }
    sum == num as u64
}

// 高分1
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = ((num as f64).log10() + 1.).floor() as u32;
    (0..digits)
        .map(|i| (num / 10u32.pow(i) % 10).pow(digits))
        .sum::<u32>() == num
}

// 高分 2
pub fn is_armstrong_number3(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;
    let sum = num_str.chars()
        // 将一个字符串数转换为10 进制数字 c.to_digit(10)
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(num_len))
        .sum();
    num == sum
}


#[test]
fn test_zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
#[ignore]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
#[ignore]
fn test_there_are_no_2_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
#[ignore]
fn test_three_digit_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
#[ignore]
fn test_three_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
#[ignore]
fn test_four_digit_armstrong_number() {
    assert!(is_armstrong_number(9474))
}

#[test]
#[ignore]
fn test_four_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9475))
}

#[test]
#[ignore]
fn test_seven_digit_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
#[ignore]
fn test_seven_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9_926_316))
}
