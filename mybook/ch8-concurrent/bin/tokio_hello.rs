use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    hello1().await;
    hello2().await
}

async fn hello1() {
    for i in 1..5 {
        sleep(Duration::from_secs(1));
        println!("hello1--{}", i)
    }
}

async fn hello2() {
    for i in 1..5 {
        sleep(Duration::from_secs(1));
        println!("hello2=={}", i)
    }
}

// 并没有异步打印，因为没有task。或者说在同一个task中。
