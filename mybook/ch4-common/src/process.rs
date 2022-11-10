use std::process;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn process_() {
    // 挂起进程 kill  -SIGSTOP pid
    // 继续进程 kill  -SIGCONT pid
    let delay = Duration::from_secs(1);
    let pid = process::id();
    println!("pid:{}", pid);
    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}

fn func1() {
    unsafe {
        //" 0表示正常退出 "
        exit(0)
    }
}

fn func2() {
    unsafe { exit(0) }
}
