//指示
// 实现一个处理没有日期的时间的时钟。
//
// 您应该能够添加和减去分钟。
//
// 表示同一时间的两个时钟应该彼此相等。
//
// 您还需要.to_string()为Clock结构实现。我们将使用它来显示时钟的状态。您可以通过直接实现它或使用Display trait来实现。
//
// 你.to_string()为Clock结构实现了吗？
//
// 如果是这样，请尝试实现 Display trait for Clock。
//
// 特征允许以一种通用的方式来实现各种类型的功能。
//
// 如需进一步学习，请考虑如何String::from为该Clock类型实现。您不必实际实现这一点——它是多余的Display，当目标类型是时，这通常是更好的选择String——但在你的工具包中拥有一些类型转换特征是有用的。
#[allow(unused)]
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = chrono::NaiveTime::from_hms(self.h as u32, self.m as u32, 0)
            .format("%H:%M")
            .to_string();
        write!(f, "{}", s)
    }
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        handle_t(h, m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        handle_t(self.h, self.m + minutes)
    }
}

fn handle_t(h: i32, m: i32) -> Clock {
    let mut m = m;
    let mut h = h;
    while m < 0 {
        m = m + 60;
        h -= 1
    }
    while h < 0 {
        h += 24
    }
    h = h + m / 60;
    if h >= 24 {
        h %= 24;
    }
    m %= 60;
    Clock { h: h, m: m }
}

#[test]
fn demo() {
    println!("{}", Clock::new(-83, 49));
    assert_eq!(Clock::new(10, 37).add_minutes(1), Clock::new(34, 38))
    // assert_eq!(Clock::new(-83, 49), Clock::new(34, 37))
}

// 高星答案

//use std::fmt;
// const DAY: i64 = 24 * 60;
// const HOUR: i64 = 60;
// #[derive(Debug, Eq, PartialEq)]
// pub struct Clock {
//     minutes: i64,
// }
// impl Clock {
//     pub fn new(hours: i64, minutes: i64) -> Clock {
//         Clock {
//             minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY
//         }
//     }
//     pub fn add_minutes(self, minutes: i64) -> Clock {
//         Clock::new(0, self.minutes + minutes)
//     }
// }
// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
//     }
// }
