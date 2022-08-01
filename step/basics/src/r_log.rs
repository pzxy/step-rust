// 首先在Cargo.toml中引入 log="4.0"

// log trait
// #![allow(unused)]
// fn main() {
//     pub trait Log: Sync + Send {
//         fn enabled(&self, metadata: &Metadata<'_>) -> bool;
//         fn log(&self, record: &Record<'_>);
//         fn flush(&self);
//     }
// }

// enabled 判断日志级别能否被记录，log_enabled！宏使用。
// log 会记录 record 所代表的日志。
// flush 会将缓存中的日志刷到标准输出或者文件中。


// log！宏，需要手动设置日志级别
pub fn log1() {
    use log::{log, Level};
    let data = (42, "Forty-two");
    let private_data = "private";
    log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
    data.0, data.1, private_data);
}

// log_enabled! 宏

pub fn log2() {
    use log::{log, Level};
    let data = (42, "Forty-two");
    let private_data = "private";
    log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
    data.0, data.1, private_data);
}