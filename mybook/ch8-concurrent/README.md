[参考链接](https://time.geekbang.org/column/article/445814)

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
oneshot：写一次，读一次。一次性。实现（atomic + swap ）
rendezvous：不发数据，channel size为0。实现（Mutex + Condvar）
bounded：类似go中channel，有队列，队列写满，写者被挂起。当阻塞后，若读取数据，Condvar 的 notify_one 通知写者，继续写入。实现(mutex + condvar)，go中ring。
unbounded：没上限，自动扩容。

[实现一个mpsc](../bin/mpsc.rs)：多生产者，单消费者。缓存大小无上限。测试驱动开发。
[完整代码实现](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=042ee12817442a32bcfa05e31a1084f9)










