[参考链接](https://time.geekbang.org/column/article/445814)

目的是先用起来，这章没必要纠结各种channel的实现原理，只要明白rust的异步原理就可以。
# CAS
[原子操作](./bin/atomic.rs)
用处：锁的基本结构，无锁结构等
坏处：原理通过cpu空转等条件合适，如果保护临界区很大，会性能差。浪费cpu。
实际的Mutex的实现，是通过信号量。不符合条件放到队列中，收到解锁信号，拿出来一个执行。pv

# Condvar （cond + var）
[condvar](./bin/condvar.rs)
相当与go中的 cond,用法是一样的，只不过rust里面的锁是自动解锁，这个cond基本不怎么用的。

# Channel
rust中4种channel

- oneshot：写一次，读一次。一次性。实现（atomic + swap ）

- rendezvous：不发数据，channel size为0。实现（Mutex + Condvar）

- bounded：类似go中channel，有队列，队列写满，写者被挂起。当阻塞后，若读取数据，Condvar 的 notify_one 通知写者，继续写入。实现(mutex + condvar)，go中ring。

- unbounded：没上限，自动扩容。

上面这几个了解就行，用的最多的是mpsc和oneshot。
值的一提的是，只有调度器中实现了future的channel阻塞时才会让出线程使用权，原生的channel
在async中使用的时候，是不会让出使用权的。而在go中我们不用担心这么多，直接将协程当作轻量级线程来用就行了。

[实现一个mpsc](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=042ee12817442a32bcfa05e31a1084f9)

# 简单原理：
目前rust中异步实现主要用的reactor，所以就只讨论reactor的实现。
## 1. 基本逻辑
1. 执行器执行task，轮询task中多个future。
2. future会返回(成功，或者不成功)。返回成功则这个future执行结束，如果返回不成功，则让出线程给其他task。
3. 监听事件(来自操作系统比如epoll，或者其他事件)。监听到后，通知执行器重新执行该future。重复2，3。

## 2. 名词解释：
1. Task

   Task在rust标准库中提供，相当于go语言中go关键字后的函数体(不完全一样，可以先这么理解)。里面组合了多个Future或者单个Future。

2. Future

   Future是将来要执行的逻辑，他是一个trait在rust标准包中。只有实现了future trait才能被包装成task在Executor中执行。

3. 执行器（调度器）

   执行task的工具（相当于GO语言中的GMP），rust虽然定义的future和task trait，但是没有提供执行他们的工具，这个工具要用户自己实现。好在官方给我们实现了一个简单的执行器`futures`。还有tokio也是一个执行器。无论是哪个，都支持reactor事件模型（以后会支持别的事件模型比如Preactor）。


[tokio 教程](https://course.rs/async-rust/tokio/overview.html)

# async/await 是什么？

这两个都是关键字。

async关键字在编译时会给这个方法实现Future trait。

.await是调用Future，主要是为了直观的异步编程。下面这两种写法是一个意思。
都是对future执行依赖的编排，可以看出使用.await更直观。
```rust
  let future = id_rpc(&my_server).and_then(|id|{
        get_row(id)
    }).map(|row|{
        json::encode(row)
    }).and_then(|encoded|{
        write_string(my_socket,encoded)
    });
```

```rust
    let id = id_rpc(&my_server).await;
    let row = get_row(id).await;
    let encoded = json::encode(row);
    write_string(my_socket,encoded).await;
```

