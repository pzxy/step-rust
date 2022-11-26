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
1. 执行器轮询多个task。
2. Task会返回(成功，或者不成功)。返回成功则task执行结束，如果返回不成功，将task放回。
3. 监听事件(来自操作系统比如epoll，或者其他事件)。监听到后，通知执行器重新执行该task。重复2，3。
## 2. 名词解释：
1. Task

   Task在rust标准库中提供，相当于go语言中go关键字后的函数体(不完全一样，可以先这么理解)。里面有一个Future。

2. Future

   Future是将来要执行的逻辑，他是一个trait在rust标准包中。只有实现了future的trait才能被包装成task在Executor中执行。
   future中可以包含其他的一个或者多个future。

3. 执行器（调度器）

   执行task的工具（相当于GO语言中的GMP），rust虽然定义的future trait，但是没有提供执行他们的工具，这个工具要用户自己实现。好在官方给我们实现了一个简单的执行器`futures`。还有tokio也是一个执行器。无论是哪个，都支持reactor事件模型（以后会支持别的事件模型比如Preactor）。


# 详细流程梳理

1. 首先是调度器，他有一个channel A。调度器启动后会持续不断的从A中获取task，然后执行task的poll方法。task其实就是包装了一下future，task中有 channel A 和future。

2. 当我们执行调度器的spawn方法传入一个future时，调度器会包装这个future成为一个task，然后将这个task发送到A中。

3. 调度器会持续不断的从A中获取task，然后执行task的poll方法。task的poll方法，其实就是创建一个waker，将waker放入Context上下文中，然后将Context作为参数传入task中future的poll方法中。这样future中就有了waker，最后调用future的poll方法。
```rust
// 这是task的poll方法
   fn poll(self: Arc<Self>) {
        // 基于 Task 实例创建一个 waker, 它使用了之前的 `ArcWake`
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        // 没有其他线程在竞争锁时，我们将获取到目标 future
        let mut future = self.future.try_lock().unwrap();
        // 对 future 进行 poll
        let _ = future.as_mut().poll(&mut cx);
    }
```

4. 最后开始调用future的poll方法。
   如果成功就继续future的下个状态，如果不成功就返回pending状态，从而改变状态机的状态，然后调用 waker的 wake_by_ref()，这个方法会将task从新放入Channel A中。这里调用waker的 wake_by_ref()方法的契机是用户可以自己把握。比如我们让延迟3秒后调用 wake_by_ref()，那么就开启一个线程，睡眠3秒后再调用wake()这样，该task3秒后会被发到A中，然后调度器从A中取出来再执行这个task中的future的poll方法(也就是重复步骤3，4)。

由于这个waker是通用的，所以官方提供了ArcWake trait，只要我们实现里面的wake_by_ref方法就行了。
```rust
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
       // 这里就是将task发送到 channel A中
        arc_self.schedule();
    }
}
```

只是做了大概梳理，更具体的细节要自己看了。



# async/await 是什么？

这两个都是关键字。

async关键字在编译时会给这个方法实现Future trait。

.await是调用Future，主要是为了直观的异步编程。下面这两种写法是一个意思。
都是对future执行依赖的编排，可以看出使用.await更直观。
```rust
//如果我们不使用.await就需要这样写。
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

