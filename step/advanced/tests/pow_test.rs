use std::fs::read_to_string;

use sha256::digest;

#[test]
fn pow_demo() {
    let mut input = "hello";
    let val = digest(&digest(input));
    let mut nonce = 0;
    while verify(&val, String::from("0")) {
        nonce += 1;
        input = input + nonce.to_string();
        let val = digest(&digest(input));
    }
    println!("{}", input);
}

fn verify(a: &str, target: String) -> bool {
    let ac: Vec<char> = a.chars().collect();
    for (i, v) in target.chars().enumerate() {
        if ac[i] != v {
            return false;
        }
    }
    true
}