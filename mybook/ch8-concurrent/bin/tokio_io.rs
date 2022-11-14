use std::thread::sleep;
use std::time::Duration;

use tokio::fs::{File, OpenOptions};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "./mybook/ch8-concurrent/text.txt";
    write_file(path).await?;
    // copy会覆盖掉文件的前面内容。
    copy(path).await?;
    read_file(path).await
}

async fn copy(path: &str) -> io::Result<()> {
    // 将 reader 拷贝到 writer中。
    let mut reader: &[u8] = b"hello stitch";
    let mut f = open_file(path).await?;
    // 可以直接拷贝socket
    // io::copy(&mut socket, &mut socket).await
    //AsyncRead  AsyncWrite  TcpStream，File，Stdout , Vec<u8>、&[u8]  这些结构实现了 reader 和 writer
    io::copy(&mut reader, &mut f).await?;
    Ok(())
}

async fn write_file(path: &str) -> io::Result<()> {
    let mut f = open_file(path).await.unwrap();
    let context: &[u8] = b"hello world  , i am lonely";
    f.write_all(context).await
}

async fn read_file(path: &str) -> io::Result<()> {
    let mut f = open_file(path).await.unwrap();
    // 读指定长度(字节)
    // let mut buf = [0; 10];
    // let n = f.read(&mut buf[..]).await?;
    // println!("read context: {:?}", &buf[0..n]);

    // 读到全部内容(字节)
    // let mut buf = Vec::new();
    // f.read_to_end(&mut buf).await?;

    let mut buf = String::new();
    f.read_to_string(&mut buf).await?;
    println!("read context: {:?}", buf);
    Ok(())
}

async fn open_file(path: &str) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(false)
        .open(path)
        .await
}
