// 这里虽然也是一个文件，但是不是一个crate，只有tests目录下的一级文件才是crate
// 可以创建一个方法给其他测试crate使用


use std::io;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn _setup() {
    println!("setup")
}

// tokio 读取指定长度内容
#[tokio::main]
pub async fn async_read() -> io::Result<()> {
    let mut f = File::open("./foo.txt").await?;
    let mut buf = [0; 10];
    f.read(&mut buf).await?;
    println!("{}", String::from_utf8(Vec::from(buf)).unwrap());
    Ok(())
}

// 读取全部内容
#[tokio::main]
pub async fn async_read_end() -> io::Result<()> {
    let mut f = File::open("./foo.txt").await?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).await?;
    println!("{}", String::from_utf8(buf).expect("Found invalid UTF8"));
    Ok(())
}

#[tokio::main]
pub async fn async_write() {
    let mut f = File::open("foo.txt").await.unwrap();
    //  b"some bytes" 是什么意思。这种写法可以将一个 &str 字符串转变成一个字节数组：&[u8;10]，
    // 然后 write 方法又会将这个 &[u8;10] 的数组类型隐式强转为数组切片: &[u8]。会扩容，如果大小大于10，也是没问题的。
    let n = f.write(b"hello world every day hello world every day hello world every day").await.unwrap();
    println!("write n:{}", n)
}

