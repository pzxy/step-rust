// 当程序发生panic后，有两种处理方案
// 1. 沿着函数调用栈往回走，清理每个函数中的数据
// 默认就是这种情况
// 2. 直接中断进程，让操作系统清理。
// 这种情况需要在Cargo.toml中设置:
// [profile.release]
// panic = `abort`

use std::fs::File;
use std::io::{Error,ErrorKind, Read};

/*
panic(unwrap expect) 使用场景：
1. demo
2. 原型代码
3. 测试
4. 确定一定是对的，unwrap
*/
fn main(){
    r_panic();
    // r_match();
    r_error_kind();
    r_unwrap();
    r_unwrap_expect();
    read_username_from_file().expect("panic file");
    read_username_from_file2().expect("panic file2");
}
fn r_panic() {
    // 设置环境变量 RUST_BACKTRACE=1 会显示调用栈，同时Cargo run 命令不能带有 --release参数
    panic!("crash and burn")
}

// 处理Result方式1,match方式
#[allow(unused)]
fn r_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error)
        }
    };
}

// 处理不同类型的错误
fn r_error_kind() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // 错误类型，继续判断
        Err(err) => match err.kind() {
            // 如果是因为文件没找到而发生错误，就创建这个文件，再判断错误
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file:{:?}", e),
            },
            // 这个表示其他错误，这个变量可以随便写。
            e => panic!("Error creating file:{:?}", e),
        }
    };
}

// unwrap_or_else 闭包处理
fn r_unwrap() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file:{:?}", error);
            })
        } else {
            panic!("Error creating file:{:?}", error);
        }
    });
}

// unwrap expect 处理错误区别
fn r_unwrap_expect() {
    // unwrap 相当于以下代码，但是不能自定义错误信息
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error)
    //     }
    // };
    let f = File::open("hello.txt").unwrap();
    // expect 可以自定义错误信息
    let f = File::open("hello.txt").expect("expect可以自定义错误信息");
}

// 错误作为函数返回值
fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("text.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 快捷处理错误，？其实是调用from函数，将一种错误转换成另外一种错误类型。
// 如果发生错误就会返回，否则继续执行
// 如果在main函数中使用？，则fn main() -> Result<(),Box<dyn Error>>
fn read_username_from_file2() -> Result<String, Error> {
    let mut f = File::open("text.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // 或者这样写，效果一样
    // let mut s = String::new();
    // File::open("text.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}

// 23/8/30
// unwrap & expect 都是错误以后直接报错。
// ? 返回错误或者结果。
// 只要结构体实现了 io::Error trait 就可以当错误struct来用。

