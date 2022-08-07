//! Rust异步笔记

use std::thread;

use std::time::Duration;

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
///  async 标记的语法块在编译的时候,会被装换成实现Future 的状态机,与同步调用的阻塞当前
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
struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    thread::sleep(Duration::from_secs(3));
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    // fixme 这里只有两个任务都执行完毕,才会打印出来. 虽然我加了随眠,但是打印出来的顺序并没有变.
    // fixme 这是因为本身就是这样的,结果一定是顺序的,只是执行的过程中,在睡眠的时候,其实已经执行了
    // fixme dance 了. 这个例子是不明显的,如果我们用写io 文件,就能看出来差距了.
    // 与 go 协程调度的对比:
    // go 中直接就和线程一样了,结果是不保证顺序的,rust这种调度结果是保证顺序的.
    // 这是在当前线程中呢.
    // 20220801 理解
    // go 的协程和这个其实本质上一样,之所以没有使用 go 关键字的效果.
    // 是因为go 的运行时是自己控制的. 而 rust 是我们自己的控制的
    // go 进行了抽象,协程直接抽象成了线程来用了. 用户不用关心到底开启了几个线程.
    // 直接将go 协程理解成线程就行了.
    // 而 rust , 线程就是线程,协程就是协程,这里之所以会阻塞是因为在同一个线程中,
    // 如果开启一个线程单独 join,那么就不会阻塞了,代码会继续往下走.这是我们自己控制的
    // 而 go 是 go 调度器已经封装了自己控制了.
    futures::join!(f1, f2);
}
