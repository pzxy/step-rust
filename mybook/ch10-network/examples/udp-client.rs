use std::error::Error;
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_addr: SocketAddr = String::from("127.0.0.1:8988").parse()?;

    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    }
    .parse()?;

    // 只使用一个链接
    // let socket = UdpSocket::bind(local_addr).await?;
    // socket.connect(&remote_addr).await?;
    for i in 1..=10 {
        let socket = UdpSocket::bind(local_addr).await?;
        socket.connect(&remote_addr).await?;
        // 发送数据
        let mut data = format!("发送数据:{}", i.to_string());
        if i == 10 {
            data = "".to_string(); //最后一次发一个空过去
        };
        socket.send(data.as_bytes()).await?;
        // 读取数据
        const MAX_DATAGRAM_SIZE: usize = 65_507;
        let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
        let len = socket.recv(&mut data).await?;
        println!(
            "{} - Received {} bytes:{}",
            socket.local_addr().unwrap(),
            len,
            String::from_utf8_lossy(&data[..len])
        );
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
