use chrono::{DateTime, Local, NaiveDateTime};

#[test]
fn time_() {
    // 获取本地时间 doc: https://crates.io/crates/chrono
    let now: DateTime<Local> = Local::now();
    println!("now: {:?}", now); // 当前时间
    println!("timestamp: {:?}", now.timestamp()); // 转时间戳

    // 当前时间格式化
    let t = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("time: {:?}", t);

    // 字符串转时间对象
    let t1 = DateTime::parse_from_str("2014-5-7T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z").unwrap();
    println!("DateTime: {:?}", t1);
    let t2 = NaiveDateTime::parse_from_str("2022-02-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("NaiveDateTime: {:?}", t2);
}
