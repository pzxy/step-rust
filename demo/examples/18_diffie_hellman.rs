//Instructions
// Diffie-Hellman key exchange.
//
// Alice and Bob use Diffie-Hellman key exchange to share secrets. They start with prime numbers, pick private keys, generate and share public keys, and then generate a shared secret key.
//
// Step 0
// The test program supplies prime numbers p and g.
//
// Step 1
// Alice picks a private key, a, greater than 1 and less than p. Bob does the same to pick a private key b.
//
// Step 2
// Alice calculates a public key A.
//
// A = g**a mod p
// Using the same p and g, Bob similarly calculates a public key B from his private key b.
//
// Step 3
// Alice and Bob exchange public keys. Alice calculates secret key s.
//
// s = B**a mod p
// Bob calculates
//
// s = A**b mod p
// The calculations produce the same result! Alice and Bob now share secret s.
//
// One possible solution for this exercise is to implement your own modular exponentiation function. To learn more about it refer to the following page.

//指示
// Diffie-Hellman 密钥交换。
//
// Alice 和 Bob 使用 Diffie-Hellman 密钥交换来共享秘密。他们从质数开始，挑选私钥，生成和共享公钥，然后生成共享密钥。
//
// 步骤 0
// 测试程序提供素数 p 和 g。
//
// 步骤1
// Alice 选择一个大于 1 且小于 p 的私钥 a。Bob 做同样的事情来选择一个私钥 b。
//
// 第2步
// Alice 计算出一个公钥 A。
//
// A = g**a mod p
// 使用相同的 p 和 g，Bob 类似地从他的私钥 b 计算出公钥 B。
//
// 第 3 步
// Alice 和 Bob 交换公钥。Alice 计算密钥 s。
//
// s = B**a mod p
// 鲍勃计算
//
// s = A**b mod p
// 计算产生相同的结果！Alice 和 Bob 现在共享秘密 s。

// 得出私钥 a 和 b
pub fn private_key(p: u64) -> u64 {
    p - 1
}
// 通过私钥a得出公钥 A ,通过私钥 b 得出公钥 B
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // 产生一个公钥
    // p 是一个质数, a 是一个大于1 和 小于 p 的质数,a 是私钥, g 是另外一个质数
    if a >= u32::MAX as u64 {
        let a2 = a - u32::MAX as u64;
        println!("{}", a2);
        (g.pow(u32::MAX) * g.pow(a2 as u32)) % p
    } else {
        g.pow(a as u32) % p
    }
}
// 得出共享秘钥, b_pub 其实是 A^B ,这样任何一个
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // 获取共享秘钥
    b_pub.pow(a as u32) % p
}

#[test]
fn test_private_key_in_range_key() {
    let primes: Vec<u64> = vec![
        5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];
    let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();
    for i in 0..primes.len() {
        assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
    }
}
#[test]
#[ignore]
fn test_public_key_correct() {
    let p: u64 = 23;
    let g: u64 = 5;
    let private_key: u64 = 6;
    let expected: u64 = 8;
    assert_eq!(public_key(p, g, private_key), expected);
}
#[test]
#[ignore]
fn test_secret_key_correct() {
    let p: u64 = 11;
    let private_key_a = 7;
    let public_key_b = 8;
    let secret = secret(p, public_key_b, private_key_a);
    let expected = 2;
    assert_eq!(secret, expected);
}
#[test]
#[ignore]
fn test_public_key_correct_big_numbers() {
    let p: u64 = 4_294_967_299;
    let g: u64 = 8;
    let private_key: u64 = 4_294_967_296;
    let expected: u64 = 4096;
    assert_eq!(public_key(p, g, private_key), expected);
}
#[test]
#[ignore]
fn test_secret_key_correct_big_numbers() {
    let p: u64 = 4_294_967_927;
    let private_key_a = 4_294_967_300;
    let public_key_b = 843;
    let secret = secret(p, public_key_b, private_key_a);
    let expected = 1_389_354_282;
    assert_eq!(secret, expected);
}

// two biggest 64bit primes
#[cfg(feature = "big-primes")]
const PRIME_64BIT_1: u64 = 0xFFFF_FFFF_FFFF_FFC5;
#[cfg(feature = "big-primes")]
const PRIME_64BIT_2: u64 = 0xFFFF_FFFF_FFFF_FFAC;
#[cfg(feature = "big-primes")]
const PRIVATE_KEY_64BIT: u64 = 0xFFFF_FFFF_FFFF_FFC3;
#[cfg(feature = "big-primes")]
const PUBLIC_KEY_64BIT: u64 = 0xB851_EB85_1EB8_51C1;
#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_public_key_correct_biggest_numbers() {
    assert_eq!(
        public_key(PRIME_64BIT_1, PRIME_64BIT_2, PRIVATE_KEY_64BIT),
        PUBLIC_KEY_64BIT
    );
}
#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_secret_key_correct_biggest_numbers() {
    let private_key_b = 0xEFFF_FFFF_FFFF_FFC0;
    let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);
    let expected_b = 4_340_425_873_327_658_043;
    assert_eq!(public_key_b, expected_b);
    let expected_key = 12_669_955_479_143_291_250;
    let secret_key = secret(PRIME_64BIT_1, public_key_b, PRIVATE_KEY_64BIT);
    assert_eq!(secret_key, expected_key);
    let secret_key = secret(PRIME_64BIT_1, PUBLIC_KEY_64BIT, private_key_b);
    assert_eq!(secret_key, expected_key);
}
#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_changed_secret_key_biggest_numbers() {
    let private_key_a = private_key(PRIME_64BIT_1);
    let public_key_a = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_a);
    let private_key_b = private_key(PRIME_64BIT_1);
    let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);
    let secret_a = secret(PRIME_64BIT_1, public_key_b, private_key_a);
    let secret_b = secret(PRIME_64BIT_1, public_key_a, private_key_b);
    assert_eq!(secret_a, secret_b);
}
#[test]
#[ignore]
fn test_changed_secret_key() {
    let p: u64 = 13;
    let g: u64 = 11;
    let private_key_a = private_key(p);
    let private_key_b = private_key(p);
    let public_key_a = public_key(p, g, private_key_a);
    let public_key_b = public_key(p, g, private_key_b);
    // Key exchange
    let secret_a = secret(p, public_key_b, private_key_a);
    let secret_b = secret(p, public_key_a, private_key_b);
    assert_eq!(secret_a, secret_b);
}
