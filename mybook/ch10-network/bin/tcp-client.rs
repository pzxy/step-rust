use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    //连接到server
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    //发送字符串
    for i in 1..=10 {
        stream
            .write(format!("发送的次数:{}", i.to_string()).as_bytes())
            .unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // let mut reader = BufReader::new(&stream);
        // let mut buffer = vec![];
        // reader.read_until(b'\n', &mut buffer).unwrap();
        println!(
            "Response from server:{:?}",
            str::from_utf8(&buffer).unwrap().trim_matches('\u{0}')
        );
        sleep(Duration::from_secs(1));
    }
    stream.write("".as_bytes()).unwrap();
}
