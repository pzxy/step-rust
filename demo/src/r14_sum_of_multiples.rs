//Instructions
// Given a number, find the sum of all the unique multiples of particular numbers up to but not including that number.
//
// If we list all the natural numbers below 20 that are multiples of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.
//
// The sum of these multiples is 78.
//指示
// 给定一个数字，找到特定数字的所有唯一倍数的总和，但不包括该数字 。
//
// 如果我们列出所有小于 20 且是 3 或 5 的倍数的自然数，我们会得到 3、5、6、9、10、12、15 和 18。
//
// 这些倍数之和为 78。
//[package]
// edition = "2021"
// name = "sum-of-multiples"
// version = "1.5.0"
pub fn sum_of_multiples2(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|&n| {
            for &v in factors.iter() {
                if n < v || v == 0 {
                    continue;
                }
                if n % v == 0 {
                    return true;
                }
            }
            false
        })
        .sum()
}

//高分
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|&f| f != 0 && i % f == 0))
        .sum()
}

#[test]
fn no_multiples_within_limit() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    assert_eq!(0, sum_of_multiples(1, &[3, 5]))
}

#[test]
#[ignore]
fn one_factor_has_multiples_within_limit() {
    assert_eq!(3, sum_of_multiples(4, &[3, 5]))
}

#[test]
#[ignore]
fn more_than_one_multiple_within_limit() {
    assert_eq!(9, sum_of_multiples(7, &[3]))
}

#[test]
#[ignore]
fn more_than_one_factor_with_multiples_within_limit() {
    assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}

#[test]
#[ignore]
fn each_multiple_is_only_counted_once() {
    assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
}

#[test]
#[ignore]
fn a_much_larger_limit() {
    assert_eq!(233_168, sum_of_multiples(1000, &[3, 5]))
}

#[test]
#[ignore]
fn three_factors() {
    assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
}

#[test]
#[ignore]
fn factors_not_relatively_prime() {
    assert_eq!(30, sum_of_multiples(15, &[4, 6]))
}

#[test]
#[ignore]
fn some_pairs_of_factors_relatively_prime_and_some_not() {
    assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
}

#[test]
#[ignore]
fn one_factor_is_a_multiple_of_another() {
    assert_eq!(275, sum_of_multiples(51, &[5, 25]))
}

#[test]
#[ignore]
fn much_larger_factors() {
    assert_eq!(2_203_160, sum_of_multiples(10_000, &[43, 47]))
}

#[test]
#[ignore]
fn all_numbers_are_multiples_of_1() {
    assert_eq!(4950, sum_of_multiples(100, &[1]))
}

#[test]
#[ignore]
fn no_factors_means_an_empty_sum() {
    assert_eq!(0, sum_of_multiples(10_000, &[]))
}

#[test]
#[ignore]
fn the_only_multiple_of_0_is_0() {
    assert_eq!(0, sum_of_multiples(1, &[0]))
}

#[test]
#[ignore]
fn the_factor_0_does_not_affect_the_sum_of_multiples_of_other_factors() {
    assert_eq!(3, sum_of_multiples(4, &[3, 0]))
}

#[test]
#[ignore]
fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
    assert_eq!(39_614_537, sum_of_multiples(10_000, &[2, 3, 5, 7, 11]))
}