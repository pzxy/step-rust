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

use log::{debug, error, info, log, log_enabled, Level};
// 实现自己的日志库
use log::{LevelFilter, SetLoggerError};
use log::{Metadata, Record};

#[test]
pub fn logger() {
    // 直接使用log，是看不到输出的，因为这是接口，要有实现才行。这里我们使用 log4rs
    // 更多的日志框架，访问 https://github.com/rust-lang/log
    // 或者直接在代码中配置：https://docs.rs/log4rs/1.2.0/log4rs/
    // 别的参考 https://zhuanlan.zhihu.com/p/322430628
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("debug");
    info!("info");
    error!("error");
    log!(
        Level::Info,
        "log！宏，手动指定日志级别，一般不这么用，了解就行。log!(Level::Error, x)"
    );
    debug!("debug");
    if log_enabled!(Level::Info) {
        // 判断该日志级别能否被打印，如果不能被打印，就没必要花费时间走下面的日志逻辑了。
        log!(
            Level::Info,
            "log_enabled!(Level::Info) 判断该日志级别能否被打印，如果不能被打印，就没必要花费时间走下面的日志逻辑了",
        );
    }
    // 设置日志级别运行 RUST_LOG=info cargo run
}

struct SimpleLogger;
// 对于这类开发者而言，自然要实现自己的 Log 特征咯:

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

static _LOGGER: SimpleLogger = SimpleLogger;
//除此之外，我们还需要像 env_logger 一样包装下 set_logger 和 set_max_level:
pub fn _init() -> Result<(), SetLoggerError> {
    log::set_logger(&_LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
