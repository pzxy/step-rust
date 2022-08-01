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


use log::{log, log_enabled, debug, info, error, Level};
use env_logger::{Builder, Target};

pub fn log1() {
    // 直接使用log，是看不到输出的，因为这是接口，要有实现才行。这里我们使用env_logger
    // 使用env_logger日志库。Cargo.toml 中添加 log="0.4.0" env_logger = "0.9"
    // 默认是输出到标准错误，所以只能打印出来错误级别的日志信息，因为只有错误日志才会写进去。
    env_logger::init();
    // env_logger是通过环境变量设置日志级别。设置日志级别运行 RUST_LOG=info cargo run
    // 设置到标准输出。这样的话，env_logger::init();就要注释掉才行。
    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.init();
    // 注意，env_logger 必须尽可能早的初始化
    debug!("debug");
    info!("info!");
    error!("error");
    // log！宏，需要手动设置日志级别
    log!(Level::Error, "log！宏，需要手动设置日志级别，log!(Level::Error,xxxx)");
    if log_enabled!(Level::Info) {
        log!(Level::Info, "log_enabled! 宏，info日志级别才打印, if log_enabled!(Level::Info) {}","{}");
    }
    // 设置日志级别运行 RUST_LOG=info cargo run
}

// 实现自己的日志库
use log::{Record, Metadata};

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

use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;
//除此之外，我们还需要像 env_logger 一样包装下 set_logger 和 set_max_level:
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

