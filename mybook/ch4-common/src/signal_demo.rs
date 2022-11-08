use std::process;
use std::thread::sleep;
use std::time::Duration;

use libc::{exit, raise, signal, SIGINT, SIGTERM, SIGUSR1, SIG_IGN};

pub fn signal_sleep() {
    unsafe {
        // 这里的 func1 as usize 其实是函数指针。fn就是创建一个函数指针。
        // 监听信号，后面是处理。
        signal(SIGTERM, func1 as usize);
        signal(SIGUSR1, func2 as usize);
        // 忽略 SIG_INT 信号。启动后使用ctrl+c时，是没用的。
        signal(SIGINT, SIG_IGN);
    }
    let delay = Duration::from_secs(1);
    for i in 0_u8.. {
        sleep(delay);
        println!(". {}", i);
        if i > 3 {
            unsafe {
                // 给自己发送信号
                raise(SIGTERM);
            }
        }
    }
}

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
        println!(" 0表示正常退出 ");
        exit(0)
    }
}

fn func2() {
    unsafe {
        println!(" 0表示正常退出 ");
        exit(0)
    }
}
