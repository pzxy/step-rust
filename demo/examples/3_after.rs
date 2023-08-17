//指示
// 给定一个时刻，确定经过一千兆秒之后的时刻。
//
// 千兆秒是 10^9 (1,000,000,000) 秒。
//
// 如果您不确定可以执行哪些操作，请PrimitiveDateTime查看time crateCargo.toml ，它在本练习的文件中列为依赖项。

// Returns a DateTime one billion seconds after start.
#![allow(unused)]

// use std::time::SystemTime;
// use time::ext::NumericalDuration;
//
// use time::PrimitiveDateTime as DateTime;
//
// pub fn after(start: DateTime) -> DateTime {
//     start + 1e9.seconds()
//     // start + time::Duration::seconds(1e9 as i64)
//     // start.add(time::Duration::new(1e9 as i64, 0))
// }
//
// fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
//     use time::{Date, Time};
//     DateTime::new(
//         Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
//         Time::from_hms(hour, minute, second).unwrap(),
//     )
// }

extern crate chrono;

use chrono::*;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1e9 as i64)
}

pub fn after2(strat: DateTime<Utc>) -> DateTime<Utc> {
    strat + Duration::seconds(10)
}

fn main() {
    let a: DateTime<Utc> = DateTime::default();
    println!("{}", a);
    println!("{}", after(DateTime::default()));
}
