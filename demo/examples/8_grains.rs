// Instructions
// Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.
//
// There once was a wise servant who saved the life of a prince. The king promised to pay whatever the servant could dream up. Knowing that the king loved chess, the servant told the king he would like to have grains of wheat. One grain on the first square of a chess board, with the number of grains doubling on each successive square.
//
// There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so on).
//
// Write code that shows:
//
// how many grains were on a given square,
// the total number of grains on the chessboard, and
// panics with a message of "Square must be between 1 and 64" if the value is not valid

//给定棋盘上的小麦粒数加倍，计算棋盘上的小麦粒数。
//
// 从前有一位聪明的仆人救了王子的命。国王答应支付仆人所能想到的一切。知道国王喜欢下棋，仆人告诉国王他想吃小麦。棋盘的第一个方格上的一个谷物，每个连续的方格上的谷物数量加倍。
//
// 棋盘上有 64 个方格（其中方格 1 有一个grain，方格 2 有两个grains，依此类推）。
//
// 编写代码显示：
//
// 给定正方形上有多少谷物，
// 棋盘上的谷物总数，以及
// 如果值无效，则会出现“Square must be between 1 and 64”消息的恐慌
pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => {
            1u64 << (s - 1)
        }
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum()
}

fn process_square_case(input: u32, expected: u64) {
    assert_eq!(square(input), expected);
}

#[test]
/// 1
fn test_1() {
    process_square_case(1, 1);
}

#[test]
#[ignore]
/// 2
fn test_2() {
    process_square_case(2, 2);
}

#[test]
#[ignore]
/// 3
fn test_3() {
    process_square_case(3, 4);
}

#[test]
#[ignore]
/// 4
fn test_4() {
    process_square_case(4, 8);
}

//NEW
#[test]
#[ignore]
/// 16
fn test_16() {
    process_square_case(16, 32_768);
}

#[test]
#[ignore]
/// 32
fn test_32() {
    process_square_case(32, 2_147_483_648);
}

#[test]
#[ignore]
/// 64
fn test_64() {
    process_square_case(64, 9_223_372_036_854_775_808);
}

#[test]
#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_0_raises_an_exception() {
    square(0);
}

#[test]
#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_greater_than_64_raises_an_exception() {
    square(65);
}

#[test]
#[ignore]
fn test_returns_the_total_number_of_grains_on_the_board() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
}