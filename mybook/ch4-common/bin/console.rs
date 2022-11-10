use std::io;

fn main() {
    let mut read_str = String::new();
    // 从控制台读出字符串
    io::stdin().read_line(&mut read_str).expect("exception");
    // 打印内容
    println!("{}", read_str);

    // 读取命令行参数
    // let input = std::env::args().nth(1);
    // println!("input:{:?}", input);
}
