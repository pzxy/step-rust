#![cfg(not(windows))] // <1>

use libc::{SIGINT, SIGTERM, SIGTTIN, SIGUSR1, SIG_IGN};
use std::thread::sleep;
use std::time::Duration;
// 如果要修改静态变量的话，需要放到不安全代码中
static mut SHUT_DOWN: bool = false;

fn main() {
    register_signal_handlers(); // <2>

    let delay = Duration::from_secs(1);

    for i in 1_usize.. {
        //这种写法很有意思
        println!("{}", i);
        unsafe {
            // <3>
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 5 { SIGTERM } else { SIGUSR1 };

        unsafe {
            // 给自己发送信号
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {
        // 这里的 handle_sigterm as usize 其实是函数指针。fn就是创建一个函数指针。
        // 监听信号，后面是处理。
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
        // 忽略 SIG_INT 信号。启动后使用ctrl+c时，是没用的。
        libc::signal(SIGINT, SIG_IGN);
    }
}

#[allow(dead_code)] // <5>
fn handle_sigterm(_signal: i32) {
    register_signal_handlers(); // <6>

    println!("SIGTERM");

    unsafe {
        // <7>
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)] // <5>
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers(); // <6>

    println!("SIGUSR1");
}
