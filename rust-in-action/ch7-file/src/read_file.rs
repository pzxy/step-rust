// use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const BYTES_PER_LINE: usize = 16;

pub fn read_file() {
    // let arg1 = env::args().nth(1);
    // let file_name = arg1.expect("usage: fview FILENAME");
    let file_name = String::from("./src/chao.txt");
    let mut f = File::open(&file_name).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    //比chunks()更好用，如果buffer长度大于可供读取的字节数。他会返回一个错误。
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}

fn _path() {
    //std::path::Path std::path::PathBuf
    let hello = PathBuf::from("./src/chao.txt");
    let _s = hello.extension(); //返回 txt
}
