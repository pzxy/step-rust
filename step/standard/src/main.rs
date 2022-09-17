use std::io;

fn main() {
    println!("Hello, world!");
}

#[test]
fn demo_io() {
    let mut read_str = String::new();
    // 从控制台读出字符串
    io::stdin().read_line(&mut read_str).expect("exception");
    // 打印内容
    println!("{}", read_str)
}