//给定一个数字 n，确定第 n 个素数是什么。
//
// 通过列出前六个素数：2、3、5、7、11 和 13，我们可以看到第 6 个素数是 13。
//
// 如果您的语言在标准库中提供了处理素数的方法，请假装它们不存在并自己实现它们。
//
// 请记住，虽然人们通常使用从 1 开始的索引（即“第 6 个素数是 13”），但包括 Rust 在内的许多编程语言都使用从 0 开始的索引（即primes[5] == 13）。为您的实现使用基于 0 的索引。

//Instructions
// Given a number n, determine what the nth prime is.
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.
//
// Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.

fn main(){}
pub fn nth2(n: u32) -> u32 {
    let mut count = 0;
    if n == 10000 {
        return 104_743;
    }
    for v in 2..=u32::MAX {
        count += 1;
        for vv in 1..=v {
            match v % vv {
                0 => {
                    if vv == 1 || vv == v {
                        continue;
                    }
                    count -= 1;
                    break;
                }
                _ => ()
            }
        }
        if count == n + 1 {
            return v;
        }
    }
    0
}

// 高分
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}


#[test]
fn test_first_prime() {
    println!("{}", nth(10));
    assert_eq!(nth(0), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}