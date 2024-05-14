use std::process;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // 挂起进程 kill  -SIGSTOP pid
    // 继续进程 kill  -SIGCONT pid
    let delay = Duration::from_secs(1);
    let pid = process::id();
    println!("process id: {}", pid);
    for i in 1..=10 {
        sleep(delay);
        println!(". {}", i);
    }
}
