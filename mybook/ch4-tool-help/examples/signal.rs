use libc::{raise, signal, SIGINT, SIGTERM, SIGUSR1, SIG_IGN, exit};
use std::thread::sleep;
use std::time::Duration;

fn main() {
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
                raise(SIGUSR1);
            }
        }
    }
}

fn func1() {
    println!("收到 SIGTERM 信号")
}

unsafe fn func2() {
    println!("收到 SIGUSR1 信号");
    exit(1);
}
