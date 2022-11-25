//指示
// 给定一年，报告它是否是闰年。
//
// 这里的棘手之处在于公历中会出现闰年：
//
// on every year that is evenly divisible by 4
//   except every year that is evenly divisible by 100
//     unless the year is also evenly divisible by 400
// 例如，1997 年不是闰年，但 1996 年是。1900年不是闰年，但2000年是。

// Given a year, report if it is a leap year.
//
// The tricky thing here is that a leap year in the Gregorian calendar occurs:
//
// on every year that is evenly divisible by 4
//   except every year that is evenly divisible by 100
//     unless the year is also evenly divisible by 400
// For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap year, but 2000 is.

//[package]
// edition = "2021"
// name = "leap"
// version = "1.6.0"

//1582年以来的置闰规则：
// 普通闰年：公历年份是4的倍数，且不是100的倍数的，为闰年（如2004年、2020年等就是闰年）。
// 世纪闰年：公历年份是整百数的，必须是400的倍数才是闰年（如1900年不是闰年，2000年是闰年）。
fn main(){}
pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        return true;
    }
    false
}
// 高分答案

pub fn is_leap_year2(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        // 能整除 400 , 是闰年
        (0, _, _) => true,
        // 不能整除 400 ,能整除 100 整除,不是闰年,比如1900
        (_, 0, _) => false,
        // 不能能整除 400 ,不能能整除 100 , 能整除 4,是闰年
        (_, _, 0) => true,
        _ => false
    }
}


fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(is_leap_year(year), expected);
}

#[test]
fn test_year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_2_not_divisible_by_4_in_common_year() {
    process_leapyear_case(1970, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}

#[test]
#[ignore]
fn test_year_divisible_by_4_and_5_is_still_a_leap_year() {
    process_leapyear_case(1960, true);
}

#[test]
#[ignore]
fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
    process_leapyear_case(1900, false);
}

#[test]
#[ignore]
fn test_year_divisible_by_400_leap_year() {
    process_leapyear_case(2000, true);
}

#[test]
#[ignore]
fn test_year_divisible_by_400_but_not_by_125_is_still_a_leap_year() {
    process_leapyear_case(2400, true);
}

#[test]
#[ignore]
fn test_year_divisible_by_200_not_divisible_by_400_common_year() {
    process_leapyear_case(1800, false);
}

#[test]
#[ignore]
fn test_any_old_year() {
    process_leapyear_case(1997, false);
}

#[test]
#[ignore]
fn test_early_years() {
    process_leapyear_case(1, false);
    process_leapyear_case(4, true);
    process_leapyear_case(100, false);
    process_leapyear_case(400, true);
    process_leapyear_case(900, false);
}

#[test]
#[ignore]
fn test_century() {
    process_leapyear_case(1700, false);
    process_leapyear_case(1800, false);
    process_leapyear_case(1900, false);
}

#[test]
#[ignore]
fn test_exceptional_centuries() {
    process_leapyear_case(1600, true);
    process_leapyear_case(2000, true);
    process_leapyear_case(2400, true);
}

#[test]
#[ignore]
fn test_years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();
    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {:?}", incorrect_years);
    }
}