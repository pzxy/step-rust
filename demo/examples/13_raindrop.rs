//指示
// 您的任务是将数字转换为包含与某些潜在因素相对应的雨滴声音的字符串。一个因数是一个数，该数除以另一个数，没有余数。测试一个数字是否是另一个数字的最简单方法是使用模运算。
//
// 的规则raindrops是，如果给定数字：
//
// 有 3 作为因子，将“Pling”添加到结果中。
// 有 5 作为因子，将“Plang”添加到结果中。
// 有 7 作为因子，将“Plong”添加到结果中。
// 没有3、5 或 7 中的任何一个作为因子，结果应该是数字的位数。
// 例子
// 28 有 7 作为因子，但不是 3 或 5，因此结果将是“Plong”。
// 30 有 3 和 5 作为因子，但没有 7，所以结果将是“PlingPlang”。
// 34 没有被 3、5 或 7 分解，因此结果将是“34”。

// Instructions
// Your task is to convert a number into a string that contains raindrop sounds corresponding to certain potential factors. A factor is a number that evenly divides into another number, leaving no remainder. The simplest way to test if a one number is a factor of another is to use the modulo operation.
//
// The rules of raindrops are that if a given number:
//
// has 3 as a factor, add 'Pling' to the result.
// has 5 as a factor, add 'Plang' to the result.
// has 7 as a factor, add 'Plong' to the result.
// does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
// Examples
// 28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
// 30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
// 34 is not factored by 3, 5, or 7, so the result would be "34".


pub fn raindrops(n: u32) -> String {
    let mut ret = String::new();
    if n % 3 == 0 {
        ret += "Pling";
    }
    if n % 5 == 0 {
        ret += "Plang";
    }
    if n % 7 == 0 {
        ret += "Plong";
    }
    if ret.len() == 0 {
        ret += n.to_string().as_str();
    }
    ret
}
// 高分
pub fn raindrops2(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;
    let mut rez = String::new();
    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }
    if rez.is_empty() { rez = x.to_string(); }
    rez
}

#[test]
fn test_1() {
    assert_eq!("1", raindrops(1));
}

#[test]
#[ignore]
fn test_3() {
    assert_eq!("Pling", raindrops(3));
}

#[test]
#[ignore]
fn test_5() {
    assert_eq!("Plang", raindrops(5));
}

#[test]
#[ignore]
fn test_7() {
    assert_eq!("Plong", raindrops(7));
}

#[test]
#[ignore]
fn test_6() {
    assert_eq!("Pling", raindrops(6));
}

#[test]
#[ignore]
fn test_8() {
    assert_eq!("8", raindrops(8));
}

#[test]
#[ignore]
fn test_9() {
    assert_eq!("Pling", raindrops(9));
}

#[test]
#[ignore]
fn test_10() {
    assert_eq!("Plang", raindrops(10));
}

#[test]
#[ignore]
fn test_14() {
    assert_eq!("Plong", raindrops(14));
}

#[test]
#[ignore]
fn test_15() {
    assert_eq!("PlingPlang", raindrops(15));
}

#[test]
#[ignore]
fn test_21() {
    assert_eq!("PlingPlong", raindrops(21));
}

#[test]
#[ignore]
fn test_25() {
    assert_eq!("Plang", raindrops(25));
}

#[test]
#[ignore]
fn test_27() {
    assert_eq!("Pling", raindrops(27));
}

#[test]
#[ignore]
fn test_35() {
    assert_eq!("PlangPlong", raindrops(35));
}

#[test]
#[ignore]
fn test_49() {
    assert_eq!("Plong", raindrops(49));
}

#[test]
#[ignore]
fn test_52() {
    assert_eq!("52", raindrops(52));
}

#[test]
#[ignore]
fn test_105() {
    assert_eq!("PlingPlangPlong", raindrops(105));
}

#[test]
#[ignore]
fn test_3125() {
    assert_eq!("Plang", raindrops(3125));
}

#[test]
#[ignore]
fn test_12121() {
    assert_eq!("12121", raindrops(12_121));
}
