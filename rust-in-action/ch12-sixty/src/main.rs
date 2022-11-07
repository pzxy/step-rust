use std::thread::sleep;
use std::{process, time};

fn main() {
    // 挂起进程 kill  -SIGSTOP pid
    // 继续进程 kill  -SIGCONT pid
    let delay = time::Duration::from_secs(1);
    let pid = process::id();
    println!("pid:{}", pid);
    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
