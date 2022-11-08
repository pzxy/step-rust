#[test]
fn crc32() {
    let s = "this is crc32 checksum";
    let sum = crc::crc32::checksum_ieee(s.as_bytes());
    println!("crc32 sum: [0x{:08x}]", sum);
}
