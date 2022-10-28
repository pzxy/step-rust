use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);
    // take(2)只能处理两次请求
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n"; // b 转换成自己字符串
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let context = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, context);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
// 请求
// Method Request-URI HTTP-Version CRLF
// header CRLF
// message-body

// 响应
// HTTP-Version Status-Code Reason-Phrase CRLF
// header CRLF
// message-body
