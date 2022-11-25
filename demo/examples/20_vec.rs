#[test]
#[ignore]
fn test_vec() {
    let mut v: Vec<u8> = vec![107, 49, 118, 49];
    let k = v.split_off(2);
    println!("k{:?},v{:?}", k, v)
}
