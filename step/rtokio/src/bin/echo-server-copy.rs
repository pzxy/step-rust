use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            //任何一个读写器( reader + writer )都可以使用 io::split 方法进行分离，最终返回一个读取器和写入器，
            // 这两者可以独自的使用，例如可以放入不同的任务中。
            let (mut rd, mut wr) = socket.split();
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("输出到标准错误")
            }
        });
    };
}