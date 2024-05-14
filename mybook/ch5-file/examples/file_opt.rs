#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    fs::File,
    io::{BufReader, Write, BufRead},
};
use std::io::Read;

use chrono::Local;

fn main() {}
#[allow(unused)]
#[test]
fn open_or_create_file() {
    // let path = std::path::PathBuf::from("./text.txt");
    // let _s = hello.extension(); //返回 txt
    let _f1 = std::fs::File::open("./text1.txt");
    let _f2 = std::fs::OpenOptions::new()
        .read(true) // 可读
        .write(true) // 可写
        .create(true) // 不存在就创建
        .append(true) // 追加不删除
        .open("./text2.txt");
}
#[allow(unused)]
#[test]
fn write_file() {
    let mut f3 = std::fs::OpenOptions::new()
        .read(true) // 可读
        .write(true) // 可写
        .create(true) // 不存在就创建
        .append(true) // 追加不删除
        .open("./text3.txt")
        .unwrap();
    let context = Local::now().to_string();
    f3.write_all(format!("{}\n", context).as_bytes()).unwrap()
}
#[allow(unused)]
#[test]
fn read_file() {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open("./text3.txt")
        .unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{:?}", line.unwrap());
    }
}
#[allow(unused)]
#[test]
fn read_file_binary() {
    let file_name = String::from("./text3.txt");
    let mut f = File::open(&file_name).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; 16_usize];
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
        println!();
        pos += 16_usize;
    }
}
