// 当程序发生panic后，有两种处理方案
// 1. 沿着函数调用栈往回走，清理每个函数中的数据
// 默认就是这种情况
// 2. 直接中断进程，让操作系统清理。
// 这种情况需要在Cargo.toml中设置:
// [profile.release]
// panic = `abort`

/*
panic(unwrap expect) 使用场景：
1. demo
2. 原型代码
3. 测试
4. 确定一定是对的，unwrap
*/

use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

// ? 快捷处理错误，？其实是调用from函数，将一种错误转换成另外一种错误类型。
// 如果发生错误就会返回，否则继续执行
// 如果在main函数中使用？，则fn main() -> Result<(),其他类型>
fn _read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("text.txt")?;
    // 加上 ？后，上面那一行，就相当于下面这4行代码。
    // let mut f = match File::open("text.txt") {
    //     Ok(v) => v,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    let result = f.read_to_string(&mut s);
    match result {
        Ok(v) => println!("{}", v),
        // 根据 kind方法来处理不同类型的错误。
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {}
            _ => {}
        },
    }
    Ok(s)
    // 或者这样写，效果一样
    // let mut s = String::new();
    // File::open("text.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}
// unwrap和expect可以获取Result枚举中的Ok，Option枚举中的Some，但是遇到Err和None都会panic。
// unwrap
// expect，只不过可以自定义提示信息。
fn _panic() {
    // 设置环境变量 RUST_BACKTRACE=1 会显示调用栈，同时Cargo run 命令不能带有 --release参数
    panic!("crash and burn")
}
