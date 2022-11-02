// hexdump: 从文件中获取一个字节流，然后以十六进制数字对形式来输出这些字节。

use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
// 这里的r#里面的引号是不需要转义的。这里的b说明是&[u8]
const INPUT: &'static [u8] = br#"
fn main(){
    println!("Hello, world!");
}"#;

pub fn hexdump() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    //分块 ，数组中每16个长度大小为一块。
    for line in buffer.chunks(BYTES_PER_LINE) {
        // 输出当前位置信息，最多8位，不足8位，则在左侧用零填充。
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }
    Ok(())
}
