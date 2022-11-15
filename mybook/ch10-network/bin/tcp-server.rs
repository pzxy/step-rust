use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{} connect", stream.peer_addr().unwrap().to_string());
        thread::spawn(move || handle_client(stream));
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("{} disconnect", stream.peer_addr().unwrap().to_string());
            return;
        }
        stream.write(&buf[..bytes_read]).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
