//! Rust异步笔记

use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};

use futures::executor::block_on;
use futures::{join, task, TryFutureExt};

// https://www.jianshu.com/p/227be7169e72
/// 并发模型
/// 1. OS 线程, 就是操作系统的线程
/// 2. 事件驱动(Event Driver), 就是回调. js 使用
/// 3. 协成,go 使用.
/// 4. todo actor 模型,erlang 使用,不清楚原理
/// 5. async/await ,类似 协程,但是没有 go 中实现的运行时,要自己实现,推荐tokio
/// rust 提供了关键字支持,提供了必要特性支持,以下都是 async 概念

/// 概念: Future, 一个在未来某个点被调度的任务.
/// async VS 多线程
/// 多线程线程切换资源大,要用线程池,适合 CPU 密集型,也就是不经常切换线程,一直处理一个任务
/// async 适合 io 密集型,也就是不同的任务很多. 可以设置一个线程处理多个任务,因为每个任务处理的很快,
/// 这个时候如果切换线程反而浪费时间了.
/// 如果是 cpu 密集型,单独创建一个线程,比 async 模式,更好.

/// 原理:
///  async 标记的语法块在编译的时候,会被转换成实现Future 的状态机,与同步调用的阻塞当前
/// 线程不同,当 Future 执行并遇到阻塞时,他会让出当前线程的控制权,这样其他 Future 就可以
/// 在该线程中运行,(非常类似 go协程的调度模型,阻塞让其他协程来执行),这种方式不会导致当前
/// 线程的阻塞.
/// 这是需要引入 futures 这个包. 之所以默认没引入,是因为不是所有项目都需要这个,这样打包二进制更小
/// # Examples
/// ```toml
/// [dependencies]
/// futures = "0.3"
/// ```
///

async fn learn_song() -> String {
    println!("学歌:无地自容");
    String::from("无地自容")
}

async fn sing_song(s: String) {
    println!("唱歌:{}", s)
}

async fn dance() {
    println!("dance")
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

fn main() {
    // 需要注意的是，这里唱歌跳舞的执行顺序并没有变。即使使用sleep也不会变。
    // 这时因为future要返回pending才会让出线程，而thread::sleep并不是一个future，
    // 它会立即睡眠，而不会立即返回pending，从而让出线程，让跳舞去执行。
    block_on(async {
        let f1 = learn_and_sing();
        let f2 = dance();
        // 4中组合方式
        //顺序组合：f.and_then(|val| some_new_future(val))。给你一个执行 f 的 future，用它产生的 val 构建另一个 future some_new_future(val)，然后执行该 future。
        // Mapping：f.map(|val| some_new_value(val))。给你一个执行 f 的 future，产生 some_new_value(val) 的结果。
        // Joining：f.join(g)。给你一个并行执行 f 和 g 的 future，当两者都完成时，该 future 完成，同时返回两者值。
        // Selecting：f.select(g)。给你一个并行执行 f 和 g 的 future，当其中一个完成时，该 future 完成，返回它的值和另一个 future。（想要为某个 future 添加超时吗？只需为该 future 和超时 future 执行 select 即可！）
        join!(f1, f2)
    });
}
